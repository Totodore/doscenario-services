# doscenario-services
### Backend services for [Doscenario application](https://github.com/totodore/doscienario).

## Docs

High performance GRPC API to handle multi-user document edition. All changes are stored in a HashMap as a list. 
They are persisted to database when a document is idle for more than 30s or when there are more than 1000 changes to apply.
