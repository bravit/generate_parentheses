#!/bin/sh
./deps.sh
cd go
go build .
cd ../csharp
export ASPNETCORE_URLS="http://localhost:7001"
dotnet build
cd ../java
./gradlew build
cd ../kotlin
./gradlew build
cd ../rust
cargo build