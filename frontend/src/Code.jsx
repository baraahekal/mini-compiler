import React from "react";
import { CodeEditorEditable } from "react-code-editor-editable";
import "highlight.js/styles/dracula.css";

const CodeEditor = ({ code, setCode }) => (
  <CodeEditorEditable
    value={code}
    setValue={setCode}
    width="50vw"
    height="40vh"
    language="python"
    inlineNumbers
  />
);

export default CodeEditor;
