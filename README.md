# Oxide: a DICOM archive written in Rust

## Design objectives
- Single-binary deployment
- Compatible with [OHIF](https://github.com/OHIF/Viewers) DICOM viewer
- Can be run as unprivileged user
- Stretch goal: CLI query interface

## Roadmap
- [x] Read and parse a DICOM object using [dicom-rs](https://github.com/Enet4/dicom-rs)
- [ ] C-ECHO-RQ send to remote PACS is functional
- [ ] Listen for DICOM objects on a port
- [ ] C-ECHO-RQ receive
- [ ] C-ECHO-RSP send
- [ ] C-STORE implemented and working
- [ ] C-FIND implemented and working
