# Zorya Auth Service

This is a repository to serve as authentication method to a microservice based application.

## Endpoints

| uri                                   | description                                                                                 | implemented | response time |
|---------------------------------------|---------------------------------------------------------------------------------------------|-------------|---------------|
| ``/{project}/jwt/auth``               | Authenticate user in a given project.                                                       | yes         | 38ms~         |
| ``/{project}/jwt/auth/register``      | Create user to use on the authentication                                                    | no          | -             |
| ``/{project}/jwt/token/generate``     | Authenticate API in a given project. (API is a service that authentication does not expire) | no          | -             |
| ``/{project}/jwt/auth/authenticated`` | Verify if given Bare token is Authenticated.                                                | yes         | 14ms~         |
| ``/{project}/guard/can/{permission}`` | Verify if given Bare Token has the given permission.                                        | no          | -             |
