# Set variables
$remoteUser = $env:DEPLOY_USER
$remoteHost = $env:DEPLOY_HOST
$remotePath = $env:DEPLOY_REMOTE_PATH

# Validate inputs
if (-not $remoteUser -or -not $remoteHost -or -not $remotePath) {
    Write-Error "One or more required environment variables (DEPLOY_USER, DEPLOY_HOST, DEPLOY_REMOTE_PATH) are missing."
    exit 1
}

# Clean pkg directory
Write-Host "Cleaning pkg directory..."
Remove-Item -Recurse -Force ./pkg/* -ErrorAction SilentlyContinue

# Build the WASM package
Write-Host "Building WASM package..."
wasm-pack build --target web --release

# SCP the files
Write-Host "Deploying index.html..."
scp index.html "$remoteUser@$remoteHost:$remotePath/"

Write-Host "Deploying pkg/ directory..."
scp -r ./pkg "$remoteUser@$remoteHost:$remotePath/"

Write-Host "Deploying assets/ directory..."
scp -r ./assets "$remoteUser@$remoteHost:$remotePath/"

Write-Host "Deployment complete."