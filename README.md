# Sandbox API

A repo for testing different approaches to Rust web service development with a focus on best practices

## Notable Features

* Keyset pagination
* Token bucket rate limiter
* Auth0 JWK caching
* Auth0 JWT validation
* Graceful shutdown on SIGINT and SIGTERM
* RBAC route permissions

## Database Migrations

Migrations require [SQLx CLI](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)

## URL

TBD

## Authentication

User registration + login goes through Auth0. Users get a JWT token
from Auth0 to make requests with via bearer token header

## Authorization

Auth0 manages roles and permissions for users. Each API route is associated
with a permission, which are grouped into higher level roles.

## Accounts

| Name | Endpoint |
|---|---|
| List Accounts | GET /v1/accounts |
| Retrieve Account | GET /v1/accounts/:id |
| Create Account | POST /v1/accounts |
| Delete Account | DELETE /v1/accounts/:id |
| List Account Users | GET /v1/accounts/:id/users |

## Users

| Name | Endpoint |
|---|---|
| List Users | GET /v1/users |
| Retrieve User | GET /v1/users/:id |
| Create User | POST /v1/users |
| Delete User | DELETE /v1/users/:id |
| List User Accounts | GET /v1/users/:id/accounts |

## Stripe Webhooks

| Name | Endpoint |
|---|---|
| Receive Webhook | POST /v1/stripe/webhooks |

## Healthcheck

| Name | Endpoint |
|---|---|
| HTTP Healthcheck | GET /health |
