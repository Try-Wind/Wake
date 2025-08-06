extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, FnArg, ItemImpl, Pat, PatType,
};

#[proc_macro_attribute]
pub fn tool(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemImpl);

    match tool_impl(args.to_string(), input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn tool_impl(args: String, input: ItemImpl) -> syn::Result<TokenStream2> {
    let mut name = None;
    let mut description = None;
    let mut capabilities = None;

    // Robust parsing for name = "..." and description = "..."
    let args_clean = args.trim();
    
    // Handle complex descriptions with commas by finding name and description sections
    if let Some(name_start) = args_clean.find("name") {
        if let Some(name_eq) = args_clean[name_start..].find('=') {
            let after_eq = &args_clean[name_start + name_eq + 1..].trim();
            if let Some(quote_end) = after_eq[1..].find('"') {
                let name_value = &after_eq[1..quote_end + 1];
                name = Some(name_value.to_string());
            }
        }
    }
    
    if let Some(desc_start) = args_clean.find("description") {
        if let Some(desc_eq) = args_clean[desc_start..].find('=') {
            let after_eq = &args_clean[desc_start + desc_eq + 1..].trim();
            if let Some(quote_end) = after_eq[1..].find('"') {
                let desc_value = &after_eq[1..quote_end + 1];
                description = Some(desc_value.to_string());
            }
        }
    }

    if let Some(perm_start) = args_clean.find("capabilities") {
        if let Some(perm_eq) = args_clean[perm_start..].find('=') {
            let after_eq = &args_clean[perm_start + perm_eq + 1..].trim();
            if after_eq.starts_with('[') {
                if let Some(bracket_end) = after_eq.find(']') {
                    let perm_value = &after_eq[1..bracket_end];
                    capabilities = Some(perm_value.to_string());
                }
            }
        }
    }

    let name = name.ok_or_else(|| {
        syn::Error::new_spanned(&input, "Missing required 'name' attribute")
    })?;
    
    let description = description.ok_or_else(|| {
        syn::Error::new_spanned(&input, "Missing required 'description' attribute")
    })?;

    let capabilities = capabilities.unwrap_or_else(|| "".to_string());

    // Use CARGO_PKG_NAME to detect if we're inside wake-core or external
    let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap_or_default();
    let crate_name = if pkg_name == "wake-core" || pkg_name == "wake_core" {
        quote! { crate }
    } else {
        quote! { ::wake_core }
    };

    let self_ty = &input.self_ty;

    // Find the execute method and extract parameter type
    let mut execute_method = None;
    let mut execute_preview_method = None;
    let mut param_type = None;

    for item in &input.items {
        if let syn::ImplItem::Fn(method) = item {
            if method.sig.ident == "execute" {
                execute_method = Some(method);
                
                // Extract parameter type from function signature
                for input in &method.sig.inputs {
                    if let FnArg::Typed(PatType { pat, ty, .. }) = input {
                        if let Pat::Ident(ident) = pat.as_ref() {
                            if ident.ident == "params" {
                                param_type = Some(ty.as_ref());
                                break;
                            }
                        }
                    }
                }
            } else if method.sig.ident == "execute_preview" {
                execute_preview_method = Some(method);
            }
        }
    }

    let execute_method = execute_method.ok_or_else(|| {
        syn::Error::new_spanned(&input, "Expected an 'execute' method")
    })?;

    let param_type = param_type.ok_or_else(|| {
        syn::Error::new_spanned(
            execute_method,
            "Expected 'execute' method to have a 'params' parameter",
        )
    })?;

    // Parse capabilities into a vector of Capability enum variants
    let capabilities_tokens = if capabilities.is_empty() {
        quote! { &[] }
    } else {
        let perms: Vec<&str> = capabilities.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        let perm_variants: Vec<_> = perms.iter().map(|p| {
            let p_clean = p.trim();
            match p_clean {
                "Read" => quote! { #crate_name::tools::ToolCapability::Read },
                "Write" => quote! { #crate_name::tools::ToolCapability::Write },
                "Network" => quote! { #crate_name::tools::ToolCapability::Network },
                "ToolCapability::Read" => quote! { #crate_name::tools::ToolCapability::Read },
                "ToolCapability::Write" => quote! { #crate_name::tools::ToolCapability::Write },
                "ToolCapability::Network" => quote! { #crate_name::tools::ToolCapability::Network },
                _ => {
                    // Default fallback, but emit warning in generated code
                    quote! { #crate_name::tools::ToolCapability::Read }
                }
            }
        }).collect();
        quote! { &[#(#perm_variants),*] }
    };


    // Generate execute_preview method if user provided one
    let execute_preview_impl = if execute_preview_method.is_some() {
        quote! {
            async fn execute_preview(&self, parameters: Self::Params) -> Option<#crate_name::tools::ToolResult> {
                <Self>::execute_preview(self, parameters).await
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        #input
        
        // Implement ToolDescription trait from wake-llm
        impl wake_llm::ToolDescription for #self_ty {
            fn name(&self) -> &'static str {
                #name
            }

            fn description(&self) -> &'static str {
                #description
            }

            fn parameters_schema(&self) -> serde_json::Value {
                use schemars::schema_for;
                let schema = schema_for!(#param_type);
                let mut schema_value = serde_json::to_value(schema).unwrap_or_default();
                
                // Transform schema for better OpenAI API compatibility
                // Convert "type": ["integer", "null"] to "type": "integer" and handle required fields
                fn fix_schema(schema: &mut serde_json::Value) {
                    if let serde_json::Value::Object(obj) = schema {
                        // Remove JSON Schema metadata fields that LLM APIs don't expect
                        obj.remove("$schema");
                        obj.remove("title");
                        
                        // Handle properties object
                        if let Some(serde_json::Value::Object(properties)) = obj.get_mut("properties") {
                            let mut required_fields = Vec::new();
                            
                            for (field_name, field_schema) in properties.iter_mut() {
                                if let serde_json::Value::Object(field_obj) = field_schema {
                                    // Check if this field has union type with null
                                    if let Some(serde_json::Value::Array(types)) = field_obj.get("type") {
                                        if types.len() == 2 {
                                            let has_null = types.iter().any(|t| t == "null");
                                            let non_null_type = types.iter().find(|t| *t != "null");
                                            
                                            if has_null && non_null_type.is_some() {
                                                // Replace union type with single type
                                                field_obj.insert("type".to_string(), non_null_type.unwrap().clone());
                                                // Don't add to required fields (it's optional)
                                            } else {
                                                // Field is required
                                                required_fields.push(serde_json::Value::String(field_name.clone()));
                                            }
                                        } else {
                                            // Single type, field is required
                                            required_fields.push(serde_json::Value::String(field_name.clone()));
                                        }
                                    } else if field_obj.get("type").is_some() {
                                        // Single type, field is required
                                        required_fields.push(serde_json::Value::String(field_name.clone()));
                                    }
                                    
                                    // Recursively handle nested objects
                                    fix_schema(field_schema);
                                }
                            }
                            
                            // Set required fields (only non-optional ones)
                            if !required_fields.is_empty() {
                                obj.insert("required".to_string(), serde_json::Value::Array(required_fields));
                            }
                        }
                        
                        // Recursively handle other nested objects
                        for (_, value) in obj.iter_mut() {
                            if let serde_json::Value::Object(_) = value {
                                fix_schema(value);
                            }
                        }
                    }
                }
                
                fix_schema(&mut schema_value);
                schema_value
            }
        }

        // Implement Tool trait from wake-core
        #[async_trait::async_trait]
        impl #crate_name::tools::Tool for #self_ty {
            type Params = #param_type;

            fn capabilities(&self) -> &'static [#crate_name::tools::ToolCapability] {
                #capabilities_tokens
            }

            async fn execute(&self, parameters: Self::Params) -> #crate_name::tools::ToolResult {
                // Call the user-defined execute method with 'params' parameter name
                <Self>::execute(self, parameters).await
            }

            #execute_preview_impl
        }

    };

    Ok(expanded)
}