"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var fs_1 = require("fs");
var child_process_1 = require("child_process");
(0, child_process_1.spawnSync)("cd normal && cargo build --release");
(0, child_process_1.spawnSync)("cd weak && cargo build --release");
(0, fs_1.copyFileSync)(__dirname + "/../normal/target/release/league-stop.exe", __dirname + "/../deploy/league-stop.exe");
(0, fs_1.copyFileSync)(__dirname + "/../weak/target/release/league-stop-weak.exe", __dirname + "/../deploy/league-stop-weak.exe");
