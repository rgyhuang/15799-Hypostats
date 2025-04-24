import React, { useState } from "react";

export default function FormWithGetRequest() {
  const [query, setQuery] = useState("");
  const [result, setResult] = useState(null);

  const handleSubmit = async (e) => {
    e.preventDefault();
    console.log("got called lol");
    try {
      const response = await fetch("http://localhost:8080/export", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ relname: query }),
      });
      const data = await response.json();
      console.log(data);
      setResult(data);
    } catch (error) {
      console.error("Fetch error:", error);
    }
  };

  return (
    <div className="container">
      <h1 className="title">Postgres Statistic Queries</h1>
      <form onSubmit={handleSubmit} className="form">
        <input
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Table Name"
          className="input"
        />
        <button type="submit" className="button">
          Search
        </button>
      </form>
      {result && (
        <pre className="result">{JSON.stringify(result, null, 2)}</pre>
      )}
    </div>
  );
}
