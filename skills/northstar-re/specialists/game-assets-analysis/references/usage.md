# Tool selection and usage

## DirectXTex texconv (`texconv`)
- Source: https://github.com/microsoft/DirectXTex
- Usage reference: https://github.com/microsoft/DirectXTex/wiki/texconv
- Verification: Convert an owned test DDS to PNG in scratch; check dimensions and output.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## DirectXTex texdiag (`texdiag`)
- Source: https://github.com/microsoft/DirectXTex
- Usage reference: https://github.com/microsoft/DirectXTex/wiki
- Verification: Inspect metadata of a known DDS fixture; compare dimensions, format, mip count.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## JPEXS Free Flash Decompiler (`jpexs`)
- Source: https://github.com/jindrapetrik/jpexs-decompiler
- Usage reference: https://github.com/jindrapetrik/jpexs-decompiler/wiki/Commandline-arguments
- Verification: Inspect tags in a trusted SWF without playback or script execution; export permitted resources to scratch.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## C3Studio (`c3studio`)
- Source: https://github.com/abdouthematrix/C3Studio
- Usage reference: https://github.com/abdouthematrix/C3Studio
- Verification: Review project requirements, license and source; verify display of an owned C3 fixture in isolated environment.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## Tiled2Dmap (`tiled2dmap`)
- Source: https://github.com/SantasCode/Tiled2Dmap
- Usage reference: https://github.com/SantasCode/Tiled2Dmap
- Verification: Review project and dependency paths (including 7z library); verify map dimensions on an authorized fixture.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## Tiled Map Editor (`tiled`)
- Source: https://www.mapeditor.org/
- Usage reference: https://doc.mapeditor.org/
- Verification: Open a synthetic TMX map; validate tile dimensions and layer count.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## Resource Hacker (`resource-hacker`)
- Source: https://www.angusj.com/resourcehacker/
- Usage reference: https://www.angusj.com/resourcehacker/
- Verification: Open an owned PE fixture and compare version/icon resources to known values.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## 7-Zip (`7zip`)
- Source: https://www.7-zip.org/
- Usage reference: https://www.7-zip.org/faq.html
- Verification: List and test a synthetic archive with installed CLI help; check results without extraction.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

Use read-only originals and isolated offline analysis for untrusted files. Confirm current CLI syntax and architecture from upstream before use. Source projects are reference candidates, not pre-approved installers.
