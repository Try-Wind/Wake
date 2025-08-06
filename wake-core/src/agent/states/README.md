# Agent State Machine

The Agent operates as a state machine with distinct states for lifecycle management and task execution.

## State Diagram

```
                    ┌─────────────┐
                    │   Starting  │
                    └──────┬──────┘
                           │ AgentInitialized
                           ▼
                    ┌─────────────┐
             ┌─────▶│   Running   │◄─────────┐
             │      └──────┬──────┘          │
             │             │ spawn_next_step │  BrainResult (continue)
             │             │ spawn_tools     │  ToolsCompleted
             │             ▼                 │
             │      ┌─────────────┐          │ 
             │      │ Processing  │          │
             │      │  (brain/    │──────────┘
             │      │     tools)  │
             │      └──────┬──────┘
             │             │ BrainResult (pause)
             │             │ TaskCancelled
             │             │ 
             │             ▼
  user input │      ┌─────────────┐
             │      │   Paused    │
             └──────┤ (waiting    │
                    │  for user)  │
                    └─────────────┘
                           │ completion/error
                           ▼
                    ┌─────────────┐
                    │  Terminal   │
                    │ (Completed, │
                    │ Failed, or  │
                    │ Cancelled)  │
                    └─────────────┘
```

## States

- **Starting**:   Initial state during agent initialization
- **Running**:    Active state ready to process next step  
- **Processing**: Executing brain thinking or tool calls
- **Paused**:     Waiting for user input (agent decided to pause), this is skipped in the absence of controller
- **Terminal**:   Final states (Completed, Failed, Cancelled)

## Key Events

- `AgentInitialized`: Moves from Starting to Running/Paused
- `StartThinking`: Triggers brain execution (Running → Processing)
- `BrainResult`: Brain decision result (Processing → Running/Paused)
- `ToolsCompleted`: Tool execution finished (Processing → Running)
- `CancelTask`: Cancel current operation

## State Transitions

States transition based on internal events and brain decisions. The agent automatically moves between Running and Processing states during normal operation, with Paused state used when the brain decides to yield control back to the user.
