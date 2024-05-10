import React, { useRef } from 'react'; 
import Editor from '@monaco-editor/react';

const CodeEditor = ({ code, setCode }) => {
  const editorRef = useRef(null);
  const monacoRef = useRef(null);

  const options = {
    selectOnLineNumbers: true,
    roundedSelection: false,
    readOnly: false,
    cursorStyle: 'line',
    automaticLayout: false,
  };

  const compileCode = async () => {
    monacoRef.current.editor.setModelMarkers(editorRef.current.getModel(), 'owner', [{
      startLineNumber: 4,
      startColumn: 4,
      endLineNumber: 4,
      endColumn: 4,
      severity: monacoRef.current.MarkerSeverity.Error,
      message: "expected ; at this line"
    }]);
  };

  const editorDidMount = (editor, monaco) => {
    console.log('editorDidMount', editor);
    editor.focus();
    editorRef.current = editor;
    monacoRef.current = monaco;
    compileCode();
  };

  const onChange = (newValue, e) => {
    console.log('onChange', newValue, e);
    setCode(newValue);
    compileCode();
  };

  return (
    <>
      <Editor
        height={window.innerHeight / 2}
        width={window.innerWidth / 2}
        defaultLanguage="cpp"
        theme="vs-dark"
        defaultValue={code}
        // options={options}
        onChange={onChange}
        onMount={editorDidMount}
      />
    </>
    
  );
};

export default CodeEditor;