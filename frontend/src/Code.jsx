import React from "react";
import { CodeEditorEditable } from "react-code-editor-editable";
import "highlight.js/styles/gruvbox-dark.css";
import {Button} from "@nextui-org/react";

const CodeEditor = ({ code, setCode }) => (
  <CodeEditorEditable
    value={code}
    setValue={setCode}
    width="50vw"
    height="40vh"
    language="cpp"
    inlineNumbers
    caretColor="yellow"
  />
);

export default CodeEditor;
