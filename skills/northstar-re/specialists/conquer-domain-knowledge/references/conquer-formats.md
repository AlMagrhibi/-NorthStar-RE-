# Conquer format field guide (optional, version-neutral)
Community references checked 2026-11-25. No sample client was supplied for validation. Names below are candidates, not universal facts.

| Candidate | Likely role | Initial tool / evidence |
|---|---|---|
| DLL / EXE | PE libraries and programs | Detect It Easy, PE-bear, ILSpy or Ghidra after CLR classification |
| DAT / DBC | Tables, configuration, records; text, binary or transformed | ImHex, HxD, file-specific documentation |
| WDF / WDB / DNP | Candidate resource containers, not necessarily same layout | Community source, ImHex, bounded parser; WDF specifically documented |
| INI / ANI | Configuration, identifiers, animation/resource mapping | Inspect encoding and text structure first |
| C3 / DDS | Models / textures | C3Studio / DirectXTex |
| DMAP / PUL / SCENE / MSK | Candidate map, puzzle, scene or mask resources | Tiled2Dmap for documented maps, ImHex for unfamiliar variants |
| 7Z / ZIP | Standard archive if magic matches | 7-Zip list/test |
| SWF | Legacy UI or media | JPEXS static inspection |

Candidate table names: ItemType.dat (items), MagicType.dat (skills), Monster.dat (monsters), MountType.dat (mounts), LevExp.dat (level experience), GameMap.dat (map references), MapDestination.dat (destinations), Server.dat (server configuration), Shop.dat (shops), SuitType.dat (equipment sets), MyAnimate.dat (animation mapping). Verify actual presence and role; do not use this as a definitive inventory. 3DObj.ini, 3DTexture.ini, 3DEffect.ini may connect IDs to model, texture and effect resources. Compare against observed assets. Community DAT documentation distinguishes plain binary files from files transformed by game-specific ciphers; no single universal DAT decoder. No key material or protection-bypass procedures are bundled.

## Sources and usage
- https://conquer-online.github.io/wiki/ — start with Files and format index.
- https://conquer-online.github.io/wiki/files/formats/dat.html — documented DAT variants; match to evidence rather than copying assumptions.
- https://github.com/conquer-online/co2_core_dll — source reference for file formats; inspect license and commit.
- https://github.com/darkfoxdeveloper/ConquerToolsKit — community client file tools; source review before build.
- https://github.com/abdouthematrix/C3Studio — C3 viewer and WDF/TQ reader source; verify actual requirements and license.
- https://github.com/SantasCode/Tiled2Dmap — map conversion project; inspect dependency paths and validate dimensions on fixture.
- https://github.com/darkfoxdeveloper/ConquerMapViewer — optional additional map-viewer reference (not required for environment).
No community project is guaranteed compatible with every release. Review licenses and dependencies, do not run prebuilt unknown binaries, no private server access or service modification without authorization.
