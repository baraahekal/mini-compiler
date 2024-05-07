import React from "react";
import { CodeEditorEditable } from "react-code-editor-editable";
import "highlight.js/styles/gruvbox-dark.css";
<<<<<<< HEAD
=======
import { Button } from "@nextui-org/react";
>>>>>>> 9a1d92e45b0650117494da24a20124ecfa8b173c

const CodeEditor = ({ code, setCode }) => (
  <div
    style={{
      borderRadius: "10px",
      overflow: "hidden",
      boxShadow: "0px 4px 12px rgba(0, 0, 0, 0.4)",
      backgroundColor: "#361e1c",
    }}
  >
    <CodeEditorEditable
      value={code}
      setValue={setCode}
      width="800px"
      height="500px"
      language="cpp"
      inlineNumbers
      caretColor="yellow"
    />
  </div>
);

export default CodeEditor;
