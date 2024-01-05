# nasa-rs
The nasa-rs crate provides synchronous client wrappers for Nasa's Open APIs. See below for a list of [supported APIs](#Supported_APIs). If the client has not been implemented yet, feel free to open a PR! Check out [prelude.rs](https://github.com/phasewalk1/nasa-rs/blob/master/src/prelude.rs) for client implementation details.

### Example Usage
##### Using Parameter Types with `query`
`nasa-rs` exposes parameter types for each supported API, which allows for easy query building and execution with the `query` function:
> *Using the APOD API*
```Rust
use nasa_rs::clients::{Apod, ApodParams};
use nasa_rs::prelude::{Client, *};

fn main() {
    // Create a client for the APOD endpoint
    let client = Client::<Apod>::default();
    // The 'parameter type' for APOD
    let params = ApodParams::Date("2024-01-01");
    // Query the API
    let response = client.query(&params).unwrap();
}
```
##### Using the `query_with` Function to Inject Parameters
For more granular control, `nasa-rs` exposes the `query_with` function, which accepts any type that implements `QueryValues` type (see [the trait definition](https://github.com/phasewalk1/blob/master/src/prelude.rs) for more details).
> ***!warning!*** - not yet implemented
```Rust
// ...
let client = Client::<Apod>::default();
let response = client.query_with(
    vec![ ("date", "2024-01-01") ]
)
```

## Supported APIs
- [X] [Astronomy Picture of the Day (APOD)](https://apod.nasa.gov/apod/astropix.html)
- [X] [Asteroid Near-Earth Objects Web Service (NEO)](https://api.nasa.gov/neo/rest/v1/neo/browse?api_key=DEMO_KEY)
- [ ] [Space Weather Database Of Notifications, Knowledge, Information (DONKI)](https://ccmc.gsfc.nasa.gov/tools/DONKI/)
    - [ ] [Coronal Mass Ejection (CME)](https://api.nasa.gov/#donkiCME)
    - [X] [Coronal Mass Ejection (CME) Analysis](https://api.nasa.gov/#donkiCMEAnalysis)
    - [X] [Geomagnetic Storm (GST)](https://api.nasa.gov/#donkiGST)
    - [ ] [Interplanetary Shock (IPS)](https://api.nasa.gov/#donkiIPS)
    - [ ] [Solar Flare (FLR)](https://api.nasa.gov/#donkiFLR)
    - [ ] [Solar Energetic Particle (SEP)](https://api.nasa.gov/#donkiSEP)
    - [ ] [Magnetopause Crosssing (MPC)](https://api.nasa.gov/#donkiMPC)
    - [ ] [Radiation Belt Enhancement (RBE)](https://api.nasa.gov/#donkiRBE)
    - [ ] [Hight Speed Stream (HSS)](https://api.nasa.gov/#donkiHSS)
    - [ ] [WSA+EnlilSimulation](https://api.nasa.gov/#donkiWSA)
    - [ ] [Notifications](https://api.nasa.gov/#donkiNotifications)
- [ ] [Earth](https://api.nasa.gov/)
    - [ ] [Imagery](https://api.nasa.gov/)
    - [ ] [Assets](https://api.nasa.gov/)
- [ ] [EONET](https://api.nasa.gov/)
- [ ] [EPIC](https://api.nasa.gov/)
- [ ] [Exoplanet](https://api.nasa.gov/)
- [ ] [Open Science Data Repository](https://api.nasa.gov/)
    - [ ] [Study Data File API](https://api.nasa.gov/)
    - [ ] [Study Metadata API](https://api.nasa.gov/)
    - [ ] [Study Dataset Search API](https://api.nasa.gov/)
    - [ ] [Experiments, Missions, Payloads, Hardware, Vehicles, Subjects, Biospecimens (geode-py)](https://api.nasa.gov/)
- [ ] [Insight: Mars Weather Service API](https://api.nasa.gov/)
- [ ] [Mars Rover Photos](https://api.nasa.gov/)
- [ ] [NASA Image and Video Library](https://api.nasa.gov/)
- [ ] [TechTransfer](https://api.nasa.gov/)
- [ ] [Satellite Situation Center](https://api.nasa.gov/)
- [ ] [SSD/CNEOS](https://api.nasa.gov/)
    - [ ] [CAD](https://api.nasa.gov/)
    - [ ] [Fireball](https://api.nasa.gov/)
    - [ ] [Mission Design](https://api.nasa.gov/)
    - [ ] [NHATS](https://api.nasa.gov/)
    - [ ] [Scout](https://api.nasa.gov/)
    - [ ] [Sentry](https://api.nasa.gov/)
- [ ] [Techport](https://api.nasa.gov/)
- [ ] [TLE](https://api.nasa.gov/)
- [ ] [Vesta/Moon/Mars Trek WMTS](https://api.nasa.gov/)

