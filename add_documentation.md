# Handyman Marketplace - Complete Codebase Documentation

This document provides comprehensive documentation for every file in the handyman marketplace project.

## Project Structure Overview

```
handyman/
├── backend/           # Rust API server (Axum + PostgreSQL)
├── frontend/          # Yew WebAssembly single-page app
├── shared/            # Shared types between frontend and backend
└── reference/         # Reference SaaS boilerplates for learning
```

---

## Backend Files (src/)

### **backend/src/main.rs** - API Server Entry Point
**Purpose**: Main entry point for the Axum backend server
**How it works**:
- Initializes tracing for logging
- Creates PostgreSQL connection pool
- Sets up Axum routes (public & protected)
- Applies middleware stack (CORS, auth, response mapping)
- Starts server on 127.0.0.1:3000

**Relation to program**: Central hub - all HTTP requests flow through here
**Key routes**: /api/register, /api/login, /api/booking, /api/contact, /api/me

---

### **backend/src/config.rs** - Configuration Management
**Purpose**: Centralized config singleton for environment variables
**How it works**:
- Uses `OnceLock` for thread-safe lazy initialization
- Loads from .env file or environment variables
- Provides fallback defaults for development
- Panics on invalid configuration

**Relation to program**: Used by every module for DATABASE_URL, JWT keys, Stripe keys
**Security critical**: Stores sensitive credentials

---

### **backend/src/error.rs** - Error Handling
**Purpose**: Custom error types and HTTP response mapping
**How it works**:
- Defines `Error` enum for all app errors (Auth, Database, Stripe)
- Implements `IntoResponse` for Axum HTTP errors
- Maps internal errors to client-safe error codes
- Provides status codes (401, 403, 500, etc.)

**Relation to program**: Every handler returns `Result<T, Error>`
**Error flow**: Handler → Error → IntoResponse → HTTP Status + JSON

---

### **backend/src/ctx.rs** - Request Context
**Purpose**: Stores authenticated user information for each request
**How it works**:
- Contains `user_id` extracted from JWT token
- Created by `mw_ctx_resolver` middleware
- Passed to handlers via Axum extractors

**Relation to program**: Links HTTP request to database user
**Usage**: Handlers use `ctx.user_id()` to know who made the request

---

### **backend/src/token.rs** - JWT Authentication
**Purpose**: Generate and validate JWT tokens for user sessions
**How it works**:
- **Generate**: Creates JWT with username, expiration, HMAC signature
- **Validate**: Verifies signature and checks expiration
- **Format**: `base64(ident).base64(exp).base64(signature)`
- **Signing**: Uses HMAC-SHA256 with secret key + user salt

**Relation to program**: Core authentication - login creates token, middleware validates it
**Security**: Tokens expire in 24 hours, unique per user (salt-based)

---

### **backend/src/db/mod.rs** - Database Module
**Purpose**: Exports database connection functionality
**Contents**: Re-exports `create_pool` and `DbPool` from `pool.rs`

---

### **backend/src/db/pool.rs** - Connection Pool
**Purpose**: PostgreSQL connection pool using bb8
**How it works**:
- Creates pool of reusable database connections
- Uses `bb8-postgres` for async connection pooling
- Pool shared across all HTTP handlers
- Configured with max connections, timeouts

**Relation to program**: Every database query gets connection from this pool
**Performance**: Reuses connections instead of creating new ones

---

### **backend/src/models/** - Data Models

#### **models/user.rs**
**Purpose**: User account data structure
**Fields**: id, username, email, pwd_hash, token_salt, timestamps
**Relations**: Has many bookings (future: handyman profile)

#### **models/customer.rs**
**Purpose**: Customer contact information
**Fields**: id, full_name, email, phone, timestamps
**Relations**: Has many bookings

#### **models/booking.rs**
**Purpose**: Service booking data
**Fields**: id, customer_id, user_id, work_type, location, description, price, status, payment info
**Enums**: WorkType (Plumbing, Electrical, etc.), BookingStatus, PaymentStatus
**Relations**: Belongs to User and Customer

#### **models/contact.rs**
**Purpose**: Contact form submissions
**Fields**: id, name, email, message, created_at

---

### **backend/src/repositories/** - Database Operations

#### **repositories/user.rs**
**Purpose**: User CRUD operations
**Methods**:
- `create()` - Insert new user with hashed password
- `find_by_username()` - Login lookup
- `find_by_id()` - Get user by ID
**Security**: Passwords hashed with bcrypt before storage

#### **repositories/customer.rs**
**Purpose**: Customer management
**Methods**:
- `create_or_get()` - Create customer or return existing by email
- `find_by_email()` - Lookup existing customer
**Smart**: Prevents duplicate customers with same email

#### **repositories/booking.rs**
**Purpose**: Booking CRUD operations
**Methods**:
- `create()` - Create booking + customer in single transaction
- `update_payment()` - Update payment status after Stripe
- `find_by_id()`, `list_recent()` - Query bookings

**Complex logic**: Automatically creates/finds customer, calculates price by work type

#### **repositories/contact.rs**
**Purpose**: Contact form persistence
**Methods**:
- `create()` - Insert contact message
- `list_recent()` - Admin view of recent contacts

---

### **backend/src/handlers/** - HTTP Request Handlers

#### **handlers/auth.rs**
**Purpose**: Authentication endpoints
**Routes**:
- `handle_register` - POST /api/register - Create user account
- `handle_login` - POST /api/login - Verify credentials, return JWT
- `handle_logout` - POST /api/logout - Clear auth cookie
- `handle_me` - GET /api/me - Get current user info (protected)

**Flow**: Login → Verify password → Generate JWT → Set cookie → Return user

#### **handlers/booking.rs**
**Purpose**: Booking creation endpoint
**Route**: POST /api/booking
**Flow**:
1. Receive booking data from frontend
2. Create/find customer by email
3. Calculate price based on work_type
4. Insert booking with status=pending
5. Return booking_id and price

**Future**: Will integrate Stripe payment, send notifications

#### **handlers/contact.rs**
**Purpose**: Contact form submission
**Route**: POST /api/contact
**Simple**: Just stores message in database

#### **handlers/health.rs**
**Purpose**: Health check endpoints
**Routes**:
- `root` - GET / - Simple "Hello World"
- `health_check` - GET /api/health - Returns service status

---

### **backend/src/middleware/** - HTTP Middleware

#### **middleware/mw_auth.rs**
**Purpose**: JWT authentication middleware
**Components**:
- `mw_ctx_resolver` - Extracts token, validates, creates Ctx
- `mw_ctx_require` - Blocks request if no valid authentication

**Flow**: Cookie → Parse JWT → Validate signature → Lookup user → Create Ctx

#### **middleware/mw_res_map.rs**
**Purpose**: Response transformation middleware
**Function**: Maps internal errors to client-safe JSON responses
**Security**: Hides internal error details from clients

---

### **backend/src/bin/** - Executable Binaries

#### **bin/init_db.rs**
**Purpose**: Database initialization tool
**Usage**: `cargo run --bin init_db`
**Action**: Runs schema.sql to create all tables

#### **bin/create_db.rs**
**Purpose**: Database creation tool
**Usage**: Creates the `handyman` database if it doesn't exist

---

### **backend/schema.sql** - Database Schema
**Purpose**: PostgreSQL table definitions
**Tables**:
- `users` - User accounts with auth
- `customers` - Customer contact info
- `bookings` - Service bookings
- `contact_messages` - Contact form submissions

**Features**: Foreign keys, indexes, updated_at triggers

---

## Frontend Files (frontend/)

### **frontend/main.rs** - WebAssembly Entry Point
**Purpose**: Yew app initialization
**How it works**: Mounts `<App />` component to DOM
**Compiles to**: WebAssembly running in browser

---

### **frontend/app.rs** - Root Component
**Purpose**: Main app component with routing
**Contains**: Router, authentication context provider, navigation
**Routes**: /, /login, /register, /booking, /contact, /experience, /map

---

### **frontend/lib.rs** - Frontend Library
**Purpose**: Module exports for Yew components

---

### **frontend/routes.rs** - Route Definitions
**Purpose**: Defines all app routes
**Enum**: `Route` with Home, Login, Register, Booking, Contact, Experience, Map

---

### **frontend/api/mod.rs** - API Client
**Purpose**: HTTP client for backend communication
**Functions**:
- `register()` - POST /api/register
- `login()` - POST /api/login
- `create_booking()` - POST /api/booking
- `send_contact()` - POST /api/contact

**Pattern**: All functions take success/error callbacks for async handling

---

### **frontend/contexts/auth.rs** - Auth Context
**Purpose**: Global authentication state
**Provides**: Current user, login/logout functions
**Shared**: Across all components via Yew context

---

### **frontend/components/nav.rs** - Navigation Bar
**Purpose**: Top navigation with login/logout
**Styling**: Red & white theme, sticky positioning
**Dynamic**: Shows different links based on auth status

---

### **frontend/pages/** - Page Components

#### **pages/home.rs**
**Purpose**: Landing page
**Content**: Hero section, CTA button

#### **pages/login.rs**
**Purpose**: User login form
**Flow**: Username/password → API → Set auth context → Redirect

#### **pages/register.rs**
**Purpose**: User registration form
**Flow**: Username/email/password → API → Auto-login

#### **pages/booking.rs**
**Purpose**: Service booking form
**Fields**: Location, work type, description, name, email, phone
**Includes**: Stripe test payment button

#### **pages/contact.rs**
**Purpose**: Contact form
**Simple**: Name, email, message

#### **pages/map.rs**
**Purpose**: 3D Cesium map of service area
**Features**:
- Centered on São Brás de Alportel, Portugal
- 10km coverage circle
- HQ marker
- Multiple map styles (satellite, topographic, dark mode)
- Interactive controls

#### **pages/experience.rs**
**Purpose**: About/experience page (demo content)

---

### **frontend/stripe.rs** - Stripe Integration
**Purpose**: Stripe.js wrapper for payments
**Functions**: `open_stripe_checkout()` - Opens Stripe payment modal

---

### **frontend/index.html** - HTML Template
**Purpose**: Base HTML with Yew mount point, CSS, Cesium CDN

---

## Shared Files

### **shared/lib.rs** - Shared Types
**Purpose**: Types used by both frontend and backend
**Currently**: Placeholder (will contain shared DTOs, enums)

---

## Configuration Files

### **Cargo.toml** - Workspace Configuration
**Purpose**: Defines workspace with 3 members (backend, frontend, shared)
**Dependencies**: Shared deps for all crates

### **backend/Cargo.toml**
**Key deps**: axum, tokio, sqlx, bb8-postgres, bcrypt, tower-http, serde

### **frontend/Cargo.toml**
**Key deps**: yew, yew-router, gloo-net, web-sys, wasm-bindgen

### **frontend/Trunk.toml**
**Purpose**: Trunk build configuration for Yew

---

## Data Flow Examples

### **User Registration Flow**
1. Frontend: User fills register form (`pages/register.rs`)
2. Frontend: Calls `api::register()` with username/email/password
3. Backend: `handle_register()` receives request
4. Backend: `UserRepository::create()` hashes password, inserts to DB
5. Backend: Returns success
6. Frontend: Auto-calls `api::login()`
7. Backend: `handle_login()` generates JWT token
8. Backend: Sets auth cookie
9. Frontend: Updates auth context, redirects to home

### **Booking Creation Flow**
1. Frontend: User fills booking form (`pages/booking.rs`)
2. Frontend: Calls `api::create_booking()` with form data
3. Backend: `handle_booking()` receives request
4. Backend: `CustomerRepository::create_or_get()` finds/creates customer
5. Backend: `BookingRepository::create()` inserts booking with calculated price
6. Backend: Returns booking_id and price
7. Frontend: Shows success message
8. **Future**: Initiate Stripe payment, send email notifications

### **Authentication Check Flow**
1. User requests protected route (e.g., /api/me)
2. `mw_ctx_resolver` extracts JWT from cookie
3. `token::validate_token()` verifies signature and expiration
4. `UserRepository::find_by_username()` loads user
5. Creates `Ctx` with user_id
6. Handler receives `Ctx` extractor
7. Returns user data

---

## Future Marketplace Features (from business plan)

**Database additions needed**:
- `handyman_profiles` table
- `service_categories` table
- `subscription_plans` table
- `user_subscriptions` table (Stripe)
- `jobs` table (customer posts)
- `job_applications` table (handyman bids)
- `reviews` table
- `notifications` table
- `earnings` and `payouts` tables

**New handlers needed**:
- `/api/handyman/register` - Onboard handyman
- `/api/handyman/profile` - Manage profile
- `/api/handyman/subscription` - Stripe subscription
- `/api/jobs` - List/create job posts
- `/api/jobs/:id/apply` - Submit bid
- `/api/bookings/:id/review` - Rate handyman
- `/api/earnings` - Handyman earnings dashboard

**Frontend pages needed**:
- Handyman onboarding flow
- Handyman dashboard
- Job marketplace browser
- Bidding interface
- Review/rating system
- Subscription management

---

## Technology Stack

**Backend**:
- Axum 0.8 - Web framework
- Tokio - Async runtime
- PostgreSQL - Database
- bb8 - Connection pooling
- bcrypt - Password hashing
- JWT (HMAC-SHA256) - Authentication
- Stripe - Payments

**Frontend**:
- Yew 0.21 - Component framework
- WebAssembly - Runs in browser
- Yew Router - Client-side routing
- Gloo - Web APIs
- Cesium 1.115 - 3D maps
- Tailwind CSS - Styling

**DevOps**:
- Trunk - Frontend build tool
- Cargo workspace - Monorepo management
- Docker - Database (development)

---

## Environment Variables

Create `.env` file in project root:

```env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/handyman
TOKEN_KEY=your-256-bit-secret-key-here-change-in-production
STRIPE_SECRET_KEY=sk_test_your_stripe_key
```

---

## Running the Project

```bash
# Terminal 1 - Backend
cargo run --bin backend

# Terminal 2 - Frontend
cd frontend && trunk serve

# Visit http://127.0.0.1:8080
```

---

This documentation provides a complete overview of every file and how they work together in the handyman marketplace platform.
