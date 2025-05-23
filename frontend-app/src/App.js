import React, { useState } from "react";
import ClassTable from "./components/class-info";
import ColumnTabs from "./components/stats-info";
import ImportExportButton from "./components/import-export";
import ExplainButton from "./components/explain-button";
// export current instance

export default function FormWithGetRequest() {
  const [query, setQuery] = useState("");
  const [result, setResult] = useState(null);

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      const response = await fetch("http://localhost:8080/export", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ relname: query }),
      });
      const data = await response.json();
      setResult(data);
    } catch (error) {}
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
        <ImportExportButton relname={query} stats={result} />
      </form>
      {result && (
        <div className="results-container">
          <ClassTable
            data={JSON.parse(result["class_info"])}
            fullStats={result}
          />
          <ColumnTabs statsArray={result["stats_info"]} fullStats={result} />
        </div>
      )}
      <ExplainButton />
    </div>
  );
}
