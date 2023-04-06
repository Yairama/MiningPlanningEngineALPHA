
# MiningPlanningEngineALPHA
## **********************************************************************************
## NOTE: Amethyst engine has been archived, so repo must be migrated to Bevy engine
## **********************************************************************************
## A planning engine for mining engineering

<p>
  <a href="https://www.linkedin.com/in/yairama/" rel="nofollow noreferrer">
    <img src="https://i.stack.imgur.com/gVE0j.png" alt="linkedin" class="icon" width="20" height="20"> LinkedIn
  </a> &nbsp; 
  <a href="https://github.com/Yairama" rel="nofollow noreferrer">
    <img src="https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png" alt="github" class="icon" width="20" height="20"> Github
  </a> &nbsp; 
  <a href="https://gitlab.com/Yairama" rel="nofollow noreferrer">
    <img src="https://cdn-icons-png.flaticon.com/512/5968/5968853.png" alt="gitlab" class="icon" width="20" height="20"> Gitlab
  </a>
</p>



### How to build

You need to have a [git][gitLink] version control system installed.

You need to have [Rust][RustLink] installed.

[RustLink]:https://www.rust-lang.org/tools/install
[gitLink]:https://git-scm.com
First unzip the files into \MiningPlanningEngine\projects\ folder
Then run the followings commands in a console (you will need to be given access to the repository)
```
git clone https://github.com/Yairama/MiningPlanningEngine.git
```
```
cd MinningPlanningEngine
```
For Windows OS:
```
cargo run --features vulkan
```
For MAC OS:
#### NOTE: its necesary to change the library in Cargo.tom file to change the amethyst-imgui backend from vulkan to metal
```
amethyst-imgui = { version = "0.7.1", features = ["metal"] }
```

```
cargo run --features metal
```

For a release run (much faster):
```
cargo run --release --features metal/vulkan
```

If you have performed the steps correctly, the following window should open
<video src='https://user-images.githubusercontent.com/45445692/230242974-ea979fba-d829-4ce3-8071-700b6a0a503c.mp4' width=720></video>

![In-game screenshot](screenshots/2021-01-08.png)

use WASD keys for movement and mouse movement for camera view and pres ESC key to close.

### TO DO:

-Complete the GUI.

-Add Cubes maps generator

## Results
got to big_topo.DXF/ Draw DXF in the menu option, then go to parsed_cubes.csv/ Generate Cube Maps.
If you have performed the steps correctly, the following window should open:

![In-game screenshot2](screenshots/2021-01-10.png)


Hope you enjoy this project and feel free to contribute with suggestions and improvements. Thanks for your interest!

