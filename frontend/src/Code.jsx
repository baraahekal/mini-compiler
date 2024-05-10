import React, { useRef } from 'react'; 
import Editor from '@monaco-editor/react';


const CodeEditor = ({ code, setCode }) => {
  const editorRef = useRef(null);
  const monacoRef = useRef(null);

  const options = {
    fontSize: 14,
    // fontFamily: 'Courier New',
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

    monaco.editor.defineTheme('gruvbox-dark', {
      base: 'vs-dark',
      inherit: true,
      rules: [
        { token: 'comment', foreground: '7c6f64' },
        { token: 'string', foreground: 'b8bb26' },
        { token: 'keyword', foreground: 'fb4934' },
        { token: 'number', foreground: 'd3869b' },
      ],
      colors: {
        'editor.foreground': '#ebdbb2',
        'editor.background': '#282828',
        'editorCursor.foreground': '#ebdbb2',
        // 'editor.lineHighlightBackground': '#3c3836',
      },
    });
    monaco.editor.setTheme('gruvbox-dark');

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
      {/* <div style={{ borderRadius: '10px', overflow: 'hidden' }}> */}
      <Editor
        onMount={editorDidMount}
        height={window.innerHeight / 2}
        width={window.innerWidth / 2}
        defaultLanguage="cpp"
        defaultValue={code}
        options={options}
        onChange={onChange}
      />
    {/* </div> */}
    </>
    
  );
};

export default CodeEditor;