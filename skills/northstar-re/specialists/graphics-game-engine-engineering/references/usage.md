# Graphics API and asset pipeline diagnosis — reference and verification guide

Read exact installed-version docs before use. No automatic downloads or execution of untrusted code.

## RenderDoc (`renderdoc`)
- Official: https://renderdoc.org/
- Usage: https://renderdoc.org/docs/
- Local check: Capture a self-built triangle demo with no external service.

## PIX on Windows (`pix`)
- Official: https://devblogs.microsoft.com/pix/
- Usage: https://learn.microsoft.com/windows/pix
- Local check: Confirm compatible GPU/API; capture owned Direct3D demo.

## DirectXTex texconv (`texconv`)
- Official: https://github.com/microsoft/DirectXTex
- Usage: https://github.com/microsoft/DirectXTex/wiki/texconv
- Local check: Convert an owned test DDS to PNG in scratch; check dimensions and output.

## DirectXTex texdiag (`texdiag`)
- Official: https://github.com/microsoft/DirectXTex
- Usage: https://github.com/microsoft/DirectXTex/wiki
- Local check: Inspect metadata of a known DDS fixture; compare dimensions, format, mip count.

## C3Studio (`c3studio`)
- Official: https://github.com/abdouthematrix/C3Studio
- Usage: https://github.com/abdouthematrix/C3Studio
- Local check: Review project requirements, license and source; verify display of an owned C3 fixture in isolated environment.

## Tiled Map Editor (`tiled`)
- Official: https://www.mapeditor.org/
- Usage: https://doc.mapeditor.org/
- Local check: Open a synthetic TMX map; validate tile dimensions and layer count.

## Fixture and evidence
Locally built triangle scene renders expected output; no capture of protected online games.
Record expected versus observed results, exact tool version, exit status, source and output hashes.
