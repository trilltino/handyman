# Fly.io Deployment Readiness Plan

This plan ensures `xfhandyman` is fully configured, optimized, and ready for production deployment on Fly.io.

## 1. Process Cleanup (Completed)
- [x] **Terminated background processes**: All run_command processes have been stopped to ensure a clean slate.

## 2. Configuration & Optimization Checks
- [x] **`fly.toml`**: 
    - Configuration is valid.
    - `internal_port = 3000` matches `Dockerfile` and `AppConfig`.
    - `auto_stop_machines = false` is correctly set for reliability.
- [x] **`Dockerfile.production`**:
    - **Fixed**: Removed unsupported `--jobs` flag from `cargo leptos build`.
    - **Note**: This file builds both the API binary and the Leptos frontend.
- [x] **`.dockerignore`**:
    - **Fixed**: Added strict ignores for `target/`, `node_modules/`, and `reference/` to prevent massive build context uploads (was ~7GB, now should be <50MB).

## 3. Database & Secrets Verification
- [x] **Database**: `xftradesmen-db` is deployed and attached.
- [x] **Existing Secrets**:
    - `DATABASE_URL` (Set)
    - `STRIPE_WEBHOOK_SECRET` (Set)
- [ ] **MISSING SECRETS (Critical)**:
    - You must set these before the app can fully function (especially payments):
    ```powershell
    fly secrets set STRIPE_SECRET_KEY=sk_live_...
    fly secrets set STRIPE_PUBLIC_KEY=pk_live_...
    fly secrets set STRIPE_PRODUCT_ID=prod_...
    ```
    - Check if you need `JWT_SECRET` for authentication (verify if your auth implementation uses it, usually required for secure tokens).

## 4. Final Deployment Steps
1.  **Set Missing Secrets**: Run the specific `fly secrets set` commands for Stripe keys.
2.  **Deploy**:
    Run the deployment command. Since local Docker builds struggled with memory/context size, use the remote builder:
    ```powershell
    fly deploy
    ```
    *Note: The first time usually takes 15-20 minutes to compile Rust dependencies.*

## 5. Post-Deployment Verification
- **Health Check**: `https://xftradesmen.fly.dev/health`
- **Logs**: Run `fly logs -a xftradesmen` to monitor startup.
