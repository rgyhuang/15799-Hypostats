import { useState, useEffect } from "react";
import { Modal, Button, Form, Row, Col } from "react-bootstrap";

function EditableForm({
  statToEdit,
  whichStakind,
  stavaluesString,
  setStavaluesString,
  stanumbersString,
  setStanumbersString,
  editStat,
  setEditStat,
}) {
  if (statToEdit.startsWith("stakind")) {
    return (
      <Form onSubmit={(e) => e.preventDefault()}>
        <Form.Group as={Row} controlId="editableText">
          <Form.Label>
            <h5>{"Editing " + statToEdit}</h5>
          </Form.Label>
          <Form.Label column sm="2">
            {"stavalues" + whichStakind}
          </Form.Label>
          <Col sm="10">
            <Form.Control
              type="text"
              disabled={stavaluesString === "null"}
              value={stavaluesString}
              onChange={(e) => setStavaluesString(e.target.value)}
              placeholder="Enter statistic"
            />
          </Col>
          <Form.Label column sm="2">
            {"stanumbers" + whichStakind}
          </Form.Label>
          <Col sm="10">
            <Form.Control
              type="text"
              disabled={stanumbersString === "null"}
              value={stanumbersString}
              onChange={(e) => setStanumbersString(e.target.value)}
              placeholder="Enter statistic"
            />
          </Col>
        </Form.Group>
      </Form>
    );
  } else {
    return (
      <Form onSubmit={(e) => e.preventDefault()}>
        <Form.Group as={Row} controlId="editableText">
          <Form.Label column sm="2">
            {statToEdit}
          </Form.Label>
          <Col sm="10">
            <Form.Control
              type="text"
              value={editStat}
              onChange={(e) => setEditStat(e.target.value)}
              placeholder="Enter statistic"
            />
          </Col>
        </Form.Group>
      </Form>
    );
  }
}

export default function EditModal({
  show,
  onHide,
  stats,
  fullStats,
  statToEdit,
}) {
  const isStakind = statToEdit.startsWith("stakind");
  const whichStakind = statToEdit.charAt(statToEdit.length - 1);
  const stanumbers = stats["stanumbers" + whichStakind];
  const stavalues = stats["stavalues" + whichStakind];

  const [stavaluesString, setStavaluesString] = useState("");
  const [stanumbersString, setStanumbersString] = useState("");
  const [myStat, setMyStat] = useState("");

  useEffect(() => {
    if (statToEdit) {
      setStavaluesString(isStakind ? stavalues["data"].toString() : "");
      setStanumbersString(isStakind ? stanumbers["data"].toString() : "");
      setMyStat(JSON.stringify(stats[statToEdit]));
    }
  }, [statToEdit, stats, isStakind, stavalues, stanumbers]);

  return (
    <Modal
      show={show}
      onHide={() => onHide()}
      size="lg"
      aria-labelledby="contained-modal-title-vcenter"
      centered
    >
      <Modal.Header closeButton>
        <Modal.Title id="contained-modal-title-vcenter">
          pg_statistic Editor
        </Modal.Title>
      </Modal.Header>
      <Modal.Body>
        <EditableForm
          statToEdit={statToEdit}
          whichStakind={whichStakind}
          stavaluesString={stavaluesString}
          setStavaluesString={setStavaluesString}
          stanumbersString={stanumbersString}
          setStanumbersString={setStanumbersString}
          editStat={myStat}
          setEditStat={setMyStat}
        />
      </Modal.Body>
      <Modal.Footer>
        <Button
          onClick={() => {
            let editedStat = stats;
            if (isStakind) {
              editedStat["stanumbers" + whichStakind]["data"] =
                stanumbersString;
              editedStat["stavalues" + whichStakind]["data"] = stavaluesString;
            } else {
              editedStat[statToEdit] = JSON.parse(myStat);
            }
            const colToEdit = stats["staattnum"] - 1;
            fullStats["stats_info"][colToEdit] = JSON.stringify(editedStat);
            onHide();
          }}
        >
          Save
        </Button>
        <Button
          onClick={() => onHide()}
          style={{ border: "0px", backgroundColor: "red" }}
        >
          Cancel
        </Button>
      </Modal.Footer>
    </Modal>
  );
}
