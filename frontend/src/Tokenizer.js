import React, { useState } from 'react';
import CodeEditor from './Code';
import './Tokenizer.css';

function Tokenizer() {
    const [code, setCode] = useState('');
    const [tokens, setTokens] = useState(null);

    const tokenizeCode = async () => {
        const response = await fetch('http://localhost:3030/tokenize', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ code })
        });
        const data = await response.json();
        setTokens(data);
    };

    return (
        <div className="tokenizer-container"> {/* Add a class for the parent container */}
            {/* Use the CodeEditor component instead of TextArea */}
            <CodeEditor code={code} setCode={setCode} className="code-editor" /> {/* Add className for the CodeEditor */}
            <button onClick={tokenizeCode}>Tokenize</button>
            {tokens && (
                <div>
                    <h2>Tokenized Data:</h2>
                    <ul>
                        <li>
                            <h3>Symbols:</h3>
                            <ul>
                                {tokens.symbols.map((symbol, index) => (
                                    <li key={index}>{symbol}</li>
                                ))}
                            </ul>
                        </li>
                        <li>
                            <h3>Identifiers:</h3>
                            <ul>
                                {tokens.identifiers.map((identifier, index) => (
                                    <li key={index}>{identifier}</li>
                                ))}
                            </ul>
                        </li>
                        <li>
                            <h3>Reserved Keywords:</h3>
                            <ul>
                                {tokens.reserved_words.map((reserved_word, index) => (
                                    <li key={index}>{reserved_word}</li>
                                ))}
                            </ul>
                        </li>
                        <li>
                            <h3>Variables:</h3>
                            <ul>
                                {tokens.variables.map((variable, index) => (
                                    <li key={index}>{variable}</li>
                                ))}
                            </ul>
                        </li>
                    </ul>
                </div>
            )}
        </div>
    );
}

export default Tokenizer;
