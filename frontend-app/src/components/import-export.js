import "./import-export.css";
import React, { useState, useEffect } from "react";

function ImportExportButton({ relname, stats }) {
  const [exportStats, setExportStats] = useState(null);
  useEffect(() => {
    setExportStats(stats);
  }, [stats, relname]);

  const handleImport = (e) => {
    const file = e.target.files[0];
    console.log(file);
    if (file) {
      const reader = new FileReader();
      reader.onload = async (event) => {
        try {
          const content = event.target.result;
          const response = await fetch("http://localhost:8080/load", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: content,
          });
          const result = await response.json();
        } catch (error) {
          console.error("Error uploading file:", error);
        }
      };
      reader.readAsText(file);
    }
  };

  const handleLoad = async (e) => {
    e.preventDefault();
    try {
      const response = await fetch("http://localhost:8080/load", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(exportStats),
      });
      const data = await response.json();
      console.log(data);
    } catch (error) {}
  };

  // https://theroadtoenterprise.com/blog/how-to-download-csv-and-json-files-in-react
  const downloadFile = ({ data, fileName, fileType }) => {
    // Create a blob with the data we want to download as a file
    const blob = new Blob([data], { type: fileType });
    // Create an anchor element and dispatch a click event on it
    // to trigger a download
    const a = document.createElement("a");
    a.download = fileName;
    a.href = window.URL.createObjectURL(blob);
    const clickEvt = new MouseEvent("click", {
      view: window,
      bubbles: true,
      cancelable: true,
    });
    a.dispatchEvent(clickEvt);
    a.remove();
  };

  const handleDownload = (e) => {
    e.preventDefault();
    downloadFile({
      data: JSON.stringify(exportStats),
      fileName: "pg_export.json",
      fileType: "text/json",
    });
  };

  return (
    <>
      <div className="input-group"></div>
      <input id="file" type="file" onChange={handleImport} />

      <button className="themed-button" onClick={handleLoad}>
        Load
      </button>
      <button className="themed-button" onClick={handleDownload}>
        Download
      </button>
    </>
  );
}

export default ImportExportButton;
