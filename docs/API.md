# TyperTask-Next API Specification

The TyperTask-Next daemon exposes a RESTful API designed for local integration and secure control over Tailscale mesh networks.

## Base URL
`http://<tailscale-ip>:8080/api/v1`

---

## Endpoints

### 1. Execute Macro Script
Executes a plain-text macro script payload asynchronously or synchronously.

- **URL:** `/execute`
- **Method:** `POST`
- **Content-Type:** `application/json`

#### Request Body
```json
{
  "script_content": "DELAY 100\nMOVE 500 500\nCLICK\nTYPE \"Hello World\"",
  "intent_description": "Click center screen and type Hello World"
}
```

#### Response
- **Status:** `200 OK`
- **Body:**
```json
{
  "status": "success",
  "message": "Script executed successfully",
  "commands_executed": 4
}
```

- **Error Response (400 Bad Request / 500 Internal Error):**
```json
{
  "status": "error",
  "message": "Syntax error on line 2: invalid coordinate"
}
```
