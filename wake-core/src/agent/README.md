# Agent Architecture

The Agent system is a modular architecture for autonomous task execution with LLM interaction, tool orchestration, and external control.

## Functional Diagram

```
┌───────────────────────────────────────────────────────────────────────────────┐
│                                 AGENT CORE                                    │
├─────────────────┬───────────────────────────────────────┬─────────────────────┤
│   CONTROLLER    │              CORE MODULE              │     EVENTS I/O      │
│    (Protocol)   │            (LLM Interaction)          │   (Communication)   │
│                 │                                       │                     │
│ ┌─────────────┐ │ ┌─────────────┐   ┌─────────────────┐ │ ┌─────────────────┐ │
│ │  Protocol   │ │ │   Thinker   │   │   Tool Actions  │ │ │  Event Handlers │ │
│ │  Commands   │ │ │  (Brain)    │   │   Orchestrator  │ │ │    (Async)      │ │
│ │             │ │ │             │   │                 │ │ │                 │ │
│ │ • Cancel    │ │ │ • LLM Call  │   │ • Tool Execute  │ │ │ • StatusChanged │ │
│ │ • GetState  │ │ │ • Decision  │   │ • Result Handle │ │ │ • ToolCall*     │ │
│ │ • UserInput │ │ │ • Continue/ │   │ • Cancellation  │ │ │ • UserRequired  │ │
│ │ • Response  │ │ │   Pause     │   │ • Concurrency   │ │ │ • Permission*   │ │
│ └─────────────┘ │ └─────────────┘   └─────────────────┘ │ └─────────────────┘ │
│                 │                                       │                     │
└─────────────────┼───────────────────────────────────────┼─────────────────────┘
                  │                                       │
                  ▼                                       ▼
         ┌─────────────────┐                   ┌─────────────────┐
         │  STATE MACHINE  │◄─────────────────►│  TOOL SYSTEM    │
         │                 │                   │                 │
         │ • Starting      │                   │ • Read Tools    │
         │ • Running       │                   │ • Write Tools   │
         │ • Processing    │                   │ • Network Tools │
         │ • Paused        │                   │ • Permissions   │
         │ • Terminal      │                   │ • Validation    │
         └─────────────────┘                   └─────────────────┘
```

## Core Modules

### Controller (Protocol)
**Purpose**: External command interface and control channel
- **Commands**: Cancel, GetState, SendUserInput, Permissions
- **Responses**: Ack, State, Error
- **Communication**: Async channel-based protocol
- **Lifecycle**: Controls agent execution flow

### Brain Module (LLM Interaction) 
**Purpose**: AI decision-making and reasoning engine
- **Thinker**: Core LLM interaction logic
- **Context**: Maintains conversation trace and available tools  
- **Decision**: Determines next action (continue/pause/tool use)
- **Flow Control**: Manages autonomous vs interactive execution

### Events I/O (Communication)
**Purpose**: Asynchronous event handling and external communication
- **Internal Events**: State machine communication (`BrainResult`, `ToolsCompleted`)
- **External Events**: UI/Controller notifications (`StatusChanged`, `ToolCallStarted`)
- **User Interaction**: Input requests and permission handling
- **Event Handlers**: Pluggable async event processing

## Key Interactions

1. **Controller → Core**: Protocol commands control agent lifecycle
2. **Core → Brain**: Triggers thinking processes with current context  
3. **Brain → Tools**: Executes tool calls based on LLM decisions
4. **Core → Events**: Emits events for external consumption
5. **Events → Controller**: Handles user input and permission requests

## Concurrency Model

- **State Machine**: Single-threaded with async event handling
- **Brain Execution**: Spawned tasks with cancellation tokens
- **Tool Execution**: Concurrent tool calls with result aggregation
- **Event Emission**: Non-blocking async event distribution
- **Protocol**: Channel-based async command/response pattern

The architecture enables autonomous agent operation while maintaining external control and observability through clean interfaces.