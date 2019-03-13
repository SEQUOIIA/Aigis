import * as electron from 'electron';
import * as path from 'path';

function init() : void {
    let window = new electron.BrowserWindow({width: 1220, height: 761, minWidth: 1220, minHeight: 761, frame: false});
    console.log(process.cwd());
    window.loadURL("http://localhost:8080");
    //window.loadFile(path.join(process.cwd(), "/index.html"));
    //window.loadFile(path.join(__dirname, "../index.html"));

    window.webContents.openDevTools();
}

electron.app.on('ready', init);