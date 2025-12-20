# Deployment Fix Plan

## Problem
The deployment is failing during the Docker build process because the `cargo leptos build` command in `Dockerfile.production` contains unsupported arguments (`--jobs`). The build tool `cargo-leptos` does not accept these flags, causing the build to exit with an error.

## Solution
We need to update `Dockerfile.production` to use the correct build command for the Leptos frontend.

## Steps

1.  **Modify `Dockerfile.production`**
    - **File**: `c:\Users\isich\xfhandyman\Dockerfile.production`
    - **Action**: Locate the line running `cargo leptos build`.
    - **Change**: Remove `-- --jobs 1`.
    - **New Command**: `RUN cargo leptos build --release`

2.  **Retry Deployment**
    - Run the local deployment command again. Since the previous steps (compiling dependencies and the API binary) are cached, this should proceed quickly to the frontend build step.
    - **Command**:
      ```powershell
      flyctl deploy --local-only
      ```

3.  **Monitor Build**
    - Ensure the `cargo leptos build` step completes successfully.
    - The deployment process will then verify the image and push it to Fly.io.
