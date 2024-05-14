import React, { useEffect, useRef, useState } from 'react'; 
import Editor from '@monaco-editor/react';

const CodeEditor = () => {
  const [code, setCode] = useState("");
  const editorRef = useRef(null);
  const monacoRef = useRef(null);
  const [typingTimeout, setTypingTimeout] = useState(null);

  const options = {
    fontSize: 14,
  };

  const tokenizeCode = async () => {
    try {
      const response = await fetch("http://localhost:3030/tokenize", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ code }),
      });

      const data = await response.json();
      console.log(data);

      if (response.ok) { 
        let errors = [];
        if (typeof data === 'string') { 
          errors = [{ message: data, line: 1, column: 1 }];
        } else if (Array.isArray(data)) { 
          errors = data;
        }

        const markers = errors.map(error => {
          let errorMessage = error.message;
          try {
            const parsedMessage = JSON.parse(errorMessage);
            if (Array.isArray(parsedMessage) && parsedMessage.length > 0) {
              errorMessage = parsedMessage[0].message;
            }
          } catch (e) {
            // errorMessage is not a JSON string, leave it as is
          }

          let severity;
          switch (error.message_type) {
            case 'Warning':
              severity = monacoRef.current.MarkerSeverity.Warning;
              break;
            case 'Error':
            default:
              severity = monacoRef.current.MarkerSeverity.Error;
              break;
          }
          
          return {
            startLineNumber: error.line + 1,
            startColumn: error.column + 1,
            endLineNumber: error.line + 1,
            endColumn: error.column + 1,
            severity: severity,
            message: errorMessage
          };
        });
        if (errors[0].message === 'No errors found.') {
          monacoRef.current.editor.setModelMarkers(editorRef.current.getModel(), 'owner', []);
        } else {
          monacoRef.current.editor.setModelMarkers(editorRef.current.getModel(), 'owner', markers);
        }     
       }
    } catch (error) {
      console.error('Failed to fetch:', error);
    }
  };

  const handleChange = (newValue) => {
    setCode(newValue);

    if (editorRef.current && monacoRef.current) {
      monacoRef.current.editor.setModelMarkers(editorRef.current.getModel(), 'owner', []);
    }

    if (typingTimeout) {
      clearTimeout(typingTimeout);
    }

    setTypingTimeout(setTimeout(tokenizeCode, 1000));
  };

  const editorDidMount = (editor, monaco) => {
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
      },
    });
    monaco.editor.setTheme('gruvbox-dark');

    editorRef.current = editor;
    monacoRef.current = monaco;
  };

  return (
    <Editor
      height="90vh"
      defaultLanguage="cpp"
      options={options}
      value={code}
      onChange={handleChange}
      onMount={editorDidMount}
    />
  );
};

export default CodeEditor;