import React, { useState, useEffect } from "react";
import Tab from "react-bootstrap/Tab";
import Tabs from "react-bootstrap/Tabs";
import Table from "react-bootstrap/Table";
import sanitizeStats from "./utils";
import EditButton from "./edit-button";
import EditModal from "./edit-modal";
import "./stats-info.css";

function CreateTab(s, fullStats, idx) {
  const [modalShow, setModalShow] = useState(false);
  const [editStat, setEditStat] = useState("");

  let stats = JSON.parse(s);
  let columnId = stats["staattnum"];
  return (
    <Tab
      className="header"
      key={columnId}
      eventKey={columnId}
      title={"Column " + columnId}
    >
      <Table striped bordered hover variant="dark" responsive>
        <tbody>
          {[...sanitizeStats(stats).entries()].map(([statName, valueInfo]) => (
            <tr key={statName} className="subtable">
              <th>
                {statName}
                <EditButton
                  setModalShow={setModalShow}
                  setEditStat={setEditStat}
                  statToEdit={valueInfo[1]}
                />
              </th>
              <th>{valueInfo[0]}</th>
            </tr>
          ))}
        </tbody>
      </Table>
      <EditModal
        show={modalShow}
        onHide={() => setModalShow(false)}
        stats={stats}
        fullStats={fullStats}
        statToEdit={editStat}
        colToEdit={idx}
      />
    </Tab>
  );
}

function ColumnTabs({ statsArray, stats }) {
  return (
    <div className="right-div">
      <Tabs defaultActiveKey="1" className="mb-3">
        {statsArray.map((s, idx) => CreateTab(s, stats, idx))}
      </Tabs>
    </div>
  );
}

export default ColumnTabs;
