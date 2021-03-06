import * as fs from 'fs'
import * as path from 'path'
import Parser from './parser'
import CodeWriter from './codeWriter'

import {
  C_ARITHMETIC,
  C_PUSH,
  C_POP,
  C_LABEL,
  C_GOTO,
  C_IF,
  C_FUNCTION,
  C_RETURN,
  C_CALL
} from './constants'

const vmTranslater = () => {
  const directoryPath = process.argv[2]
  const allFiles = fs.readdirSync(path.resolve(__dirname, directoryPath))
  const files = allFiles.filter((file) => {
    return file.endsWith('.vm')
  })
  const index = directoryPath.lastIndexOf('/')
  const fileName = directoryPath.slice(index) + '.asm'
  const codeWriter = new CodeWriter(directoryPath + fileName)

  for (const file of files) {
    const filePath = directoryPath + '/' + file
    translate(fileName, filePath, codeWriter)
  }
}

const translate = (fileName: string, filePath: string, codeWriter: CodeWriter) => {
  const parser = new Parser(filePath)
  codeWriter.setFileName(fileName)

  while (parser.hasMoreCommands()) {
    switch(parser.commandType()) {
      case C_ARITHMETIC:
        const command = parser.arg1()
        if (command) {
          codeWriter.writeArithmetic(command)
        }
        break
      case C_PUSH:
      case C_POP:
        const segment = parser.arg1()
        const index = parser.arg2()

        if (index === null) {
          throw new Error('invalid index')
        }

        if (segment) {
          codeWriter.writePushPop(parser.commandType(), segment, index)
        }
        break
      case C_LABEL:
        const label = parser.arg1()
        if (!label) {
          throw new Error('invalid label')
        } 

        codeWriter.writeLabel(label)
        break
      case C_GOTO:
        const gotoLabel = parser.arg1()
        if (!gotoLabel) {
          throw new Error('invalid gotoLabel')
        }

        codeWriter.writeGoto(gotoLabel)
        break
      case C_IF:
        const ifLabel = parser.arg1()
        if (!ifLabel) {
          throw new Error('invalid ifLabel')
        }

        codeWriter.writeIf(ifLabel)
        break
      case C_FUNCTION:
        const functionName = parser.arg1()
        const numLocals = parser.arg2() ? Number(parser.arg2()) : 0
        if (functionName) {
          codeWriter.writeFunction(functionName, numLocals)
        }
        break
      case C_RETURN:
        codeWriter.writeReturn()
        break
      case C_CALL:
        const callFunctionName = parser.arg1()
        const numArgs = parser.arg2() ? Number(parser.arg2()) : 0
        if (callFunctionName) {
          codeWriter.writeCall(callFunctionName, numArgs);
        }
        break
      default:
        throw new Error('invalid commandType')
    }
    parser.advance()
  }
}

vmTranslater();
