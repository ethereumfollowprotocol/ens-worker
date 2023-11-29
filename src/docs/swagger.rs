use serde_json::{json, Result, Value};

fn openapi_spec() -> Result<Value> {
    let openapi_spec = json!({
      "openapi": "3.0.3",
      "info": {
        "title": "ENS EFP Worker",
        "description": "ENS metaservice used across the EFP ecosystem.",
        "license": {
          "name": "GPL-3.0"
        },
        "version": "1.0.0-1"
      },
      "paths": {
        "/a/{address}": {
          "get": {
            "tags": [
              "routes::address"
            ],
            "operationId": "get",
            "parameters": [
              {
                "name": "address",
                "in": "path",
                "description": "Address to lookup name data for",
                "required": true,
                "schema": {
                  "type": "string"
                }
              }
            ],
            "responses": {
              "200": {
                "description": "Successfully found address",
                "content": {
                  "application/json": {
                    "schema": {
                      "$ref": "#/components/schemas/ENSProfile"
                    }
                  }
                }
              },
              "400": {
                "description": "Invalid address.",
                "content": {
                  "application/json": {
                    "schema": {
                      "$ref": "#/components/schemas/ErrorResponse"
                    }
                  }
                }
              },
              "404": {
                "description": "No name was associated with this address.",
                "content": {
                  "application/json": {
                    "schema": {
                      "$ref": "#/components/schemas/ErrorResponse"
                    }
                  }
                }
              }
            }
          }
        },
        "/n/{name}": {
          "get": {
            "tags": [
              "routes::name"
            ],
            "operationId": "get",
            "parameters": [
              {
                "name": "name",
                "in": "path",
                "description": "Name to lookup the name data for.",
                "required": true,
                "schema": {
                  "type": "string"
                }
              }
            ],
            "responses": {
              "200": {
                "description": "Successfully found name.",
                "content": {
                  "application/json": {
                    "schema": {
                      "$ref": "#/components/schemas/ENSProfile"
                    }
                  }
                }
              },
              "404": {
                "description": "No name could be found.",
                "content": {
                  "application/json": {
                    "schema": {
                      "$ref": "#/components/schemas/ErrorResponse"
                    }
                  }
                }
              }
            }
          }
        },
        "/u/{name_or_address}": {
          "get": {
            "tags": [
              "routes::universal"
            ],
            "operationId": "get",
            "parameters": [
              {
                "name": "name_or_address",
                "in": "path",
                "description": "Name or address to lookup the name data for.",
                "required": true,
                "schema": {
                  "type": "string"
                }
              }
            ],
            "responses": {
              "200": {
                "description": "Successfully found name or address.",
                "content": {
                  "application/json": {
                    "schema": {
                      "$ref": "#/components/schemas/ENSProfile"
                    }
                  }
                }
              },
              "404": {
                "description": "No name or address could be found.",
                "content": {
                  "application/json": {
                    "schema": {
                      "$ref": "#/components/schemas/ErrorResponse"
                    }
                  }
                }
              }
            }
          }
        }
      },
      "components": {
        "schemas": {
          "ENSProfile": {
            "type": "object",
            "required": [
              "name",
              "display",
              "records",
              "chains",
              "fresh",
              "resolver",
              "errors"
            ],
            "properties": {
              "address": {
                "type": "string",
                "nullable": true
              },
              "avatar": {
                "type": "string",
                "nullable": true
              },
              "chains": {
                "type": "object",
                "additionalProperties": {
                  "type": "string"
                }
              },
              "display": {
                "type": "string"
              },
              "errors": {
                "type": "object",
                "additionalProperties": {
                  "type": "string"
                }
              },
              "fresh": {
                "type": "integer",
                "format": "int64"
              },
              "name": {
                "type": "string"
              },
              "records": {
                "type": "object",
                "additionalProperties": {
                  "type": "string"
                }
              },
              "resolver": {
                "type": "string"
              }
            }
          },
          "ErrorResponse": {
            "type": "object",
            "required": [
              "status",
              "error"
            ],
            "properties": {
              "error": {
                "type": "string"
              },
              "status": {
                "type": "integer",
                "format": "int32",
                "minimum": 0
              }
            }
          }
        }
      }
    });

    Ok(openapi_spec)
}

pub fn swagger_html_str() -> std::string::String {
    let swagger_html_string = "
      <!doctype html>
      <html lang='en'>
        <head>
          <meta charset='UTF-8' />
          <title>ENS EFP Worker</title>
          <link rel='stylesheet' type='text/css' href='https://unpkg.com/swagger-ui-dist/swagger-ui.css' />
          <style>
            *,:after,:before{box-sizing:inherit}::-webkit-scrollbar{height:.3rem;width:0}
            ::-webkit-scrollbar-track{-ms-overflow-style:none;overflow:-moz-scrollbars-none}
            ::-webkit-scrollbar-thumb{-ms-overflow-style:none;overflow:-moz-scrollbars-none}
            @supports (scrollbar-gutter:stable){html{overflow-y:auto;scrollbar-gutter:stable}}
            html{box-sizing:border-box;overflow:-moz-scrollbars-vertical;overflow-y:scroll;margin:0;background:rgb(255,244,254)}
            .errors-wrapper{display:none!important}
          </style>
        </head>

        <body>
          <div id='swagger-ui'></div>
          <script src='https://unpkg.com/swagger-ui-dist/swagger-ui-bundle.js' charset='UTF-8'></script>
          <script src='https://unpkg.com/swagger-ui-dist/swagger-ui-standalone-preset.js' charset='UTF-8'></script>
          <script>
            window.onload = () => window.ui = SwaggerUIBundle({
              spec: __OPENAPI_JSON__,
              deepLinking: true,
              dom_id: '#swagger-ui',
              layout: 'StandaloneLayout',
              plugins: [SwaggerUIBundle.plugins.DownloadUrl],
              presets: [SwaggerUIBundle.presets.apis, SwaggerUIStandalonePreset],
            })
          </script>
        </body>
      </html>
    ";

    let openapi_spec = openapi_spec().unwrap();

    let swagger_html_string =
        swagger_html_string.replace("__OPENAPI_JSON__", &openapi_spec.to_string());

    return swagger_html_string;
}
