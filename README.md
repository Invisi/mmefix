# mmefix

## Notes
### List of GUIDs
#### Related to default sample rate, bit depth, channels
- Likely PROPVARIANT (blob) + WAVEFORMATEX
  - {33b83365-ab41-4b3b-8f32-ab8d96168070},5
  - {33b83365-ab41-4b3b-8f32-ab8d96168070},9
  - {33b83365-ab41-4b3b-8f32-ab8d96168070},10

- PROPVARIANT (blob) + WAVEFORMATEXTENSIBLE
  - {3d6e1656-2e50-4c4c-8d85-d0acae3c6c68},3
  - {624f56de-fd24-473e-814a-de40aacaed16},3
  - {e4870e26-3cc5-4cd2-ba46-ca0a9a70ed04},0
  - {f19f064d-082c-4e27-bc73-6882a1bb8e4c},0

#### Others
- "Listen to this Device"
  - Loopback device (string): {24dbb0fc-9311-4b3d-9cf0-18ff155639d4},0
  - Checkbox (PROPVARIANT + bool): {24dbb0fc-9311-4b3d-9cf0-18ff155639d4},0

- Volume
  - Volume (PROPVARIANT + ???): {9855C4CD-DF8C-449C-A181-8191B68BD06C},0
  - Muted: {9855C4CD-DF8C-449C-A181-8191B68BD06C},1

- Device type/names
  - Type (string): {a45c254e-df1c-4efd-8020-67d146a850e0},24
    - May contain stuff introduced by drivers: "HDAUDIO", "FOCUSRITEUSB", "BTHHFENUM", "USB", ...
  - User-friendly *input/output* name (string): {a45c254e-df1c-4efd-8020-67d146a850e0},2
    - Shows up in the sound settings: "Microphone", "Line In", ...
  - User-friendly *device* name (string): {026e516e-b814-414b-83cd-856d6fef4822}, 2
    - "High Definition Audio Device", "2- Focusrite USB Audio"
