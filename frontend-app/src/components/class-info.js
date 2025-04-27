import React, { useState } from "react";
import Table from "react-bootstrap/Table";
import Dropdown from "react-bootstrap/Dropdown";
import DropdownButton from "react-bootstrap/DropdownButton";
import "./class-info.css";

export default function ClassTable({ data }) {
  const [attribute, setAttribute] = useState("");

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
            <th colSpan={2}>
              <Dropdown data-bs-theme="dark">
                <Dropdown.Toggle variant="primary" id="dropdown-basic">
                  Query pg_class Attribute
                </Dropdown.Toggle>
                <Dropdown.Menu className="scrollable-menu">
                  {Object.entries(data).map(([k, v]) => (
                    <Dropdown.Item onClick={() => setAttribute(k)}>
                      {k}
                    </Dropdown.Item>
                  ))}
                </Dropdown.Menu>
              </Dropdown>
            </th>
          </tr>
          {attribute == "" ? (
            <></>
          ) : (
            <tr>
              <th>{attribute}</th>
              <th>{JSON.stringify(data[attribute])}</th>
            </tr>
          )}
        </tbody>
      </Table>
    </div>
  );
}
