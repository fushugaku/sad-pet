param([Parameter(Mandatory=$true)][string]$Path)
$ErrorActionPreference = 'Stop'
$signature = Get-AuthenticodeSignature -LiteralPath $Path
if ($signature.Status -ne 'Valid' -or $signature.SignerCertificate.Subject -notmatch 'O=Microsoft Corporation(?:,|$)') {
    throw "Invalid Microsoft signature: $Path"
}
@{
    signer = $signature.SignerCertificate.Subject
    version = (Get-Item -LiteralPath $Path).VersionInfo.FileVersion
} | ConvertTo-Json -Compress
