import "./import-export.css";
import React from "react";

function ImportExportButton({ relname }) {
  const handleImport = (e) => {
    const file = e.target.files[0];
    console.log(file);
    if (file) {
      const reader = new FileReader();
      reader.onload = async (event) => {
        try {
          const content = event.target.result;
          console.log(content);
          const response = await fetch("http://localhost:8080/load", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: content,
          });
          const result = await response.json();
          console.log(result);
        } catch (error) {
          console.error("Error uploading file:", error);
        }
      };
      reader.readAsText(file);
    }
  };

  const handleExport = async (e) => {
    e.preventDefault();
    try {
      const response = await fetch("http://localhost:8080/export", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ relname: relname }),
      });
      const data = await response.json();

      const saveResponse = await fetch("http://localhost:8080/export_dump", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      });
      const saveResult = await saveResponse.json();
      console.log(saveResult);
    } catch (error) {}
  };

  return (
    <>
      <div className="input-group"></div>
      {/* <button className="themed-button" onClick={handleExport}> */}
      <input id="file" type="file" onChange={handleImport} />
      {/* </button> */}

      <button className="themed-button" onClick={handleExport}>
        Export
      </button>
    </>
  );
}

export default ImportExportButton;
