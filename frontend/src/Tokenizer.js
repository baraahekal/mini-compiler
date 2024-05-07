import React, { useState } from "react";
import CodeEditor from "./Code";
import { Button } from "@nextui-org/react";
import "./Tokenizer.css";
import icon from "./icon.svg";

function Tokenizer() {
  const [code, setCode] = useState("");
  const [tokens, setTokens] = useState(null);

  const tokenizeCode = async () => {
    const response = await fetch("http://localhost:3030/tokenize", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ code }),
    });
    const data = await response.json();
    setTokens(data);
  };

  return (
    <div>
      <div className="header">
        <img src={icon} alt="Icon" width="40px" height="40px" />
        <h1>Mini-Compiler</h1>
      </div>
      <div className="tokenizer-container">
        <h2>Please Write Your Code here</h2>
        <CodeEditor code={code} setCode={setCode} className="code-editor" />
        <button className="Button" onClick={tokenizeCode}>
          Tokenize
        </button>
        {tokens && (
          <div className="tokenized-data">
            <h2>Tokenized Data</h2>
            <ul className="toknized-list">
              {tokens.symbols.length > 0 && (
                <li>
                  <h3>Symbols</h3>
                  <ul className="toknized-single">
                    {Array.from(tokens.symbols).map((symbol, index) => (
                      <li key={index}>{symbol}</li>
                    ))}
                  </ul>
                </li>
              )}
              {tokens.identifiers.length > 0 && (
                <li>
                  <h3>Identifiers</h3>
                  <ul className="toknized-single">
                    {Array.from(tokens.identifiers).map((identifier, index) => (
                      <li key={index}>{identifier}</li>
                    ))}
                  </ul>
                </li>
              )}
              {tokens.reserved_words.length > 0 && (
                <li>
                  <h3>Reserved Keywords</h3>
                  <ul className="toknized-single">
                    {Array.from(tokens.reserved_words).map(
                      (reserved_word, index) => (
                        <li key={index}>{reserved_word}</li>
                      )
                    )}
                  </ul>
                </li>
              )}
              {tokens.variables.length > 0 && (
                <li>
                  <h3>Variables</h3>
                  <ul className="toknized-single">
                    {Array.from(tokens.variables).map((variable, index) => (
                      <li key={index}>{variable}</li>
                    ))}
                  </ul>
                </li>
              )}
              {Object.entries(tokens.lists).length > 0 && (
                <li>
                  <h3>Lists</h3>
                  <ul className="toknized-single">
                    {Object.entries(tokens.lists).map(
                      ([listName, listItems], index) => (
                        <li key={index}>
                          <strong>{listName}:</strong> {listItems.join(", ")}
                        </li>
                      )
                    )}
                  </ul>
                </li>
              )}
              {tokens.comments.length > 0 && (
                <li>
                  <h3>Comments</h3>
                  <ul className="toknized-single">
                    {tokens.comments.map((comment, index) => (
                      <li key={index}>{comment}</li>
                    ))}
                  </ul>
                </li>
              )}
            </ul>
          </div>
        )}
      </div>
    </div>
  );
}

export default Tokenizer;
