import React, { useState, useEffect } from "react";
import {
  Modal,
  Table,
  Dropdown,
  Button,
  Row,
  Col,
  Form,
} from "react-bootstrap";
import EditButton from "./edit-button";
import "./class-info.css";

function ClassModal({
  editClassModal,
  setEditClassModal,
  numTuples,
  setNumTuples,
  classInfo,
  fullStats,
}) {
  return (
    <Modal
      show={editClassModal}
      onHide={() => setEditClassModal(false)}
      size="lg"
      aria-labelledby="contained-modal-title-vcenter"
      centered
    >
      <Modal.Header closeButton>
        <Modal.Title id="contained-modal-title-vcenter">
          Number Tuple Editor
        </Modal.Title>
      </Modal.Header>
      <Modal.Body>
        <Form onSubmit={(e) => e.preventDefault()}>
          <Form.Group as={Row} controlId="editableText">
            <Form.Label column sm="2">
              reltuples
            </Form.Label>
            <Col sm="10">
              <Form.Control
                type="text"
                value={numTuples}
                onChange={(e) => setNumTuples(e.target.value)}
                placeholder="Enter statistic"
              />
            </Col>
          </Form.Group>
        </Form>
      </Modal.Body>
      <Modal.Footer>
        <Button
          onClick={() => {
            let updatedInfo = classInfo;
            classInfo["reltuples"] = JSON.parse(numTuples);
            fullStats["class_info"] = JSON.stringify(updatedInfo);
            setEditClassModal(false);
          }}
        >
          Save
        </Button>
        <Button
          onClick={() => setEditClassModal(false)}
          style={{ border: "0px", backgroundColor: "red" }}
        >
          Cancel
        </Button>
      </Modal.Footer>
    </Modal>
  );
}

export default function ClassTable({ data, fullStats }) {
  const [attribute, setAttribute] = useState("");
  const [editClassModal, setEditClassModal] = useState(false);
  const [numTuples, setNumTuples] = useState("");

  useEffect(() => {
    setNumTuples(JSON.stringify(data["reltuples"]));
  }, [data]);

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
            <th>
              {data["reltuples"]}
              <EditButton
                setModalShow={() => setEditClassModal(true)}
                setEditStat={() => {}}
              />
            </th>
          </tr>
          <tr>
            <th colSpan={2}>
              <Dropdown data-bs-theme="dark">
                <Dropdown.Toggle variant="primary" id="dropdown-basic">
                  Query pg_class Attribute
                </Dropdown.Toggle>
                <Dropdown.Menu className="scrollable-menu">
                  {Object.entries(data).map(([k, _]) => (
                    <Dropdown.Item key={k} onClick={() => setAttribute(k)}>
                      {k}
                    </Dropdown.Item>
                  ))}
                </Dropdown.Menu>
              </Dropdown>
            </th>
          </tr>
          {attribute === "" ? (
            <></>
          ) : (
            <tr>
              <th>{attribute}</th>
              <th>{JSON.stringify(data[attribute])}</th>
            </tr>
          )}
        </tbody>
      </Table>
      <ClassModal
        editClassModal={editClassModal}
        setEditClassModal={setEditClassModal}
        numTuples={numTuples}
        setNumTuples={setNumTuples}
        classInfo={data}
        fullStats={fullStats}
      />
    </div>
  );
}
