import React from "react";
import Table from "react-bootstrap/Table";
import "./class-info.css";

function ClassTable({ data }) {
  return (
    <div className="left-div">
      <h3>Table Information</h3>
      <Table striped bordered hover variant="dark">
        <thead>
          <tr>
            <th>Table Name</th>
            <th>{data["relname"]}</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <th>Table ID</th>
            <th>{data["oid"]}</th>
          </tr>
          <tr>
            <th>Number of Columns</th>
            <th>{data["relnatts"]}</th>
          </tr>
          <tr>
            <th>Number of Tuples</th>
            <th>{data["reltuples"]}</th>
          </tr>
          <tr>
            <th colSpan={2}>Query pg_class Attribute</th>
          </tr>
        </tbody>
        {/* <tbody>
          {Object.entries(data).map(([k, v]) => (
            <tr>
              <th>{k}</th>
              <th>{JSON.stringify(v)}</th>
            </tr>
          ))}
        </tbody> */}
      </Table>
    </div>
  );
}

export default ClassTable;
