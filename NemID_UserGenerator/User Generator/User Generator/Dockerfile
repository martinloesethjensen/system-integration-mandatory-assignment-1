FROM mcr.microsoft.com/dotnet/core/aspnet:3.1-buster-slim AS base
WORKDIR /app
EXPOSE 80
EXPOSE 443

FROM mcr.microsoft.com/dotnet/core/sdk:3.1-buster AS build
WORKDIR /src
COPY ["User Generator/User Generator.csproj", "User Generator/"]
RUN dotnet restore "User Generator/User Generator.csproj"
COPY . .
WORKDIR "/src/User Generator"
RUN dotnet build "User Generator.csproj" -c Release -o /app/build

FROM build AS publish
RUN dotnet publish "User Generator.csproj" -c Release -o /app/publish

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "User Generator.dll"]