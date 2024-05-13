import React, { useState } from "react";
import CodeEditor from "./Code";
import "./Tokenizer.css";
import icon from "./icon.svg";

function Tokenizer() {
  const [curState, setCurState] = useState(0);
  const [code, setCode] = useState("");
  const [tokens, setTokens] = useState(null);
  const [showTokenizedData, setShowTokenizedData] = useState(false);

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
  const handleClick = () => {
      setShowTokenizedData(false);
      setCurState((prevState) => (prevState === 0 ? 1 : 0));

      const button = document.querySelector(".dsButtonAnim");

      if (button.classList.contains("success")) {
        button.classList.remove("success");
      } else if (button.classList.contains("error")) {
        button.classList.remove("error");
      } else {
        button.classList.add("loading");

        setTimeout(() => {
          button.classList.remove("loading");
          button.classList.add("success");  

          // Remove the success class after 2 seconds
          setTimeout(() => {
            button.classList.remove("success");
            setShowTokenizedData(true);
            tokenizeCode();
          }, 1000);

          
        }, 1500);
      }
      setTimeout(() => {
        window.scrollTo({ top: document.body.scrollHeight, behavior: "smooth" });
      }, 2300);
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
        {/* <button className="Button" onClick={tokenizeCode}>
          Tokenize
        </button> */}
        <div class="buttonContainer">
          <div class="dsButtonAnim" onClick={handleClick}>
            <span>Toknize</span>
          </div>
        </div>

        {showTokenizedData && tokens && (
        <div className="tokenized-data">
          <h2>Tokenized Data</h2>
          <ul className="toknized-list">
            {tokens.tokens.map((token, index) => (
              <li key={index}>
                {token.token_type}: {token.lexeme}
              </li>
            ))}
          </ul>
        </div>
      )}
      </div>
    </div>
  );
}

export default Tokenizer;
