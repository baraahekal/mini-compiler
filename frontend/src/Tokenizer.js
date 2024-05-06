import React, { useState } from 'react';

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
        <div>
            <textarea value={code} onChange={e => setCode(e.target.value)} />
            <button onClick={tokenizeCode}>Tokenize</button>
            {tokens && <pre>{JSON.stringify(tokens, null, 2)}</pre>}
        </div>
    );
}

export default Tokenizer;
