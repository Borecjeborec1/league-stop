import { copyFileSync } from 'fs';
import { spawnSync } from 'child_process';

copyFileSync(__dirname + "/../normal/target/release/league-stop.exe", __dirname + "/../deploy/league-stop.exe");
copyFileSync(__dirname + "/../weak/target/release/league-stop-weak.exe", __dirname + "/../deploy/league-stop-weak.exe");
copyFileSync(__dirname + "/../Readme.md", __dirname + "/../deploy/Readme.md");
copyFileSync(__dirname + "/../LICENSE.md", __dirname + "/../deploy/LICENSE.md");