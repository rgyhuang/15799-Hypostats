import React, { useState, useEffect } from "react";
import Tab from "react-bootstrap/Tab";
import Tabs from "react-bootstrap/Tabs";
import Table from "react-bootstrap/Table";
import sanitizeStats from "./utils";
import EditButton from "./edit-button";
import EditModal from "./edit-modal";
import "./stats-info.css";

function CreateTab(
  stats,
  fullStats,
  editStat,
  setEditStat,
  modalState,
  setModalState,
  histoModalState,
  setHistoModalState,
  idx
) {
  let columnId = stats["staattnum"];

  const showHistoModal = () =>
    setHistoModalState((prev) => ({ ...prev, [idx]: true }));
  const hideHistoModal = () =>
    setHistoModalState((prev) => ({ ...prev, [idx]: false }));

  return (
    <Tab
      className="header"
      key={columnId}
      eventKey={columnId}
      title={"Column " + columnId}
    >
      <Table striped bordered hover variant="dark" responsive>
        <tbody>
          {[
            ...sanitizeStats(
              stats,
              histoModalState,
              showHistoModal,
              hideHistoModal,
              idx
            ).entries(),
          ].map(([statName, valueInfo]) => (
            <tr key={statName} className="subtable">
              <th>
                {statName}
                <EditButton
                  setModalShow={() =>
                    setModalState((prev) => ({ ...prev, [idx]: true }))
                  }
                  setEditStat={() => setEditStat(valueInfo[1])}
                />
              </th>
              <th>{valueInfo[0]}</th>
            </tr>
          ))}
        </tbody>
      </Table>
      <EditModal
        show={modalState[idx] || false}
        onHide={() => setModalState((prev) => ({ ...prev, [idx]: false }))}
        stats={stats}
        fullStats={fullStats}
        statToEdit={editStat}
      />
    </Tab>
  );
}

function ColumnTabs({ statsArray, fullStats }) {
  const [modalState, setModalState] = useState({});
  const [editStat, setEditStat] = useState("");
  const [histoModalState, setHistoModalState] = useState({});
  const parsedArray = statsArray.map((s) => JSON.parse(s));

  useEffect(() => {
    setModalState((prev) => {
      const updated = { ...prev };
      for (let i = 0; i < statsArray.length; i++) {
        if (!(i in updated)) {
          updated[i] = false;
        }
      }
      return updated;
    });
    setHistoModalState((prev) => {
      const updated = { ...prev };
      for (let i = 0; i < statsArray.length; i++) {
        if (!(i in updated)) {
          updated[i] = false;
        }
      }
      return updated;
    });
  }, [statsArray]);

  return (
    <div className="right-div">
      <Tabs defaultActiveKey="1" className="mb-3">
        {parsedArray.map((stats, idx) => {
          return CreateTab(
            stats,
            fullStats,
            editStat,
            setEditStat,
            modalState,
            setModalState,
            histoModalState,
            setHistoModalState,
            idx
          );
        })}
      </Tabs>
    </div>
  );
}

export default ColumnTabs;
