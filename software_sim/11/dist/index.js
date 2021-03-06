"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    Object.defineProperty(o, k2, { enumerable: true, get: function() { return m[k]; } });
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
var fs = __importStar(require("fs"));
var path = __importStar(require("path"));
var compilationEngine_1 = __importDefault(require("./compilationEngine"));
var jackCompiler = function () {
    var directoryPath = process.argv[2];
    var allFiles = fs.readdirSync(path.resolve(__dirname, directoryPath));
    var files = allFiles.filter(function (file) {
        return file.endsWith('.jack');
    });
    for (var _i = 0, files_1 = files; _i < files_1.length; _i++) {
        var file = files_1[_i];
        var inputFilePath = directoryPath + '/' + file;
        var outputFilePath = __dirname + '/' + (directoryPath + '/' + file).slice(0, -5) + '.vm';
        new compilationEngine_1.default(inputFilePath, outputFilePath);
    }
};
jackCompiler();
