# RFC-XXX: Info Carry Rules

## Checkpoints
When determining the input checkpoint for ingest or transform operation only checkpoint in the latest `AddData` or `ExecuteQuery` are considered.

If an engine operation resulted in no updates to the checkpoint, but checkpoint is still relevant for subsequent runs - a hash of the previous checkpoint should be specified in the new `AddData` or `ExecuteQuery` metadata event.

## Watermarks
Output watermarks for root and derivative datasets function under same rules.

First several `AddData` or `ExecuteQuery` blocks may not have watermarks.

Once watermark is set in one block - all blocks that follow should either specify the same or greater output watermark.

Thus watermark is monotonically non-decreasing.