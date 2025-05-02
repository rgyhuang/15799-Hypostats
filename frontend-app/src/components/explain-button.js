import Button from "react-bootstrap/Button";
import React, { useState } from "react";
import "./explain-button.css";

const ExplainButton = () => {
  const [query, setQuery] = useState("");
  const [result, setResult] = useState([]);

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      const response = await fetch("http://localhost:8080/explain", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ query: query }),
      });

      if (!response.ok) {
        throw new Error("Failed to submit text");
      }

      const result = await response.json();
      console.log("Response from backend:", result);
      setResult(result["plan"]);
      console.log("Response from backend:", result);
    } catch (error) {
      console.error("Error:", error);
    }
  };

  return (
    <div className="explain-container">
      <form onSubmit={handleSubmit} className="form">
        <input
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Explain Query"
          className="input"
        />
        <Button type="submit" className="button">
          Search
        </Button>
      </form>
      {result.length > 0 && (
        <div className="plan-container">
          <h2>Explain Plan</h2>
          {result.map((line, idx) => (
            <pre key={idx}>{line}</pre>
          ))}
        </div>
      )}
    </div>
  );
};

export default ExplainButton;
