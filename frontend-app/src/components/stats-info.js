import Tab from "react-bootstrap/Tab";
import Tabs from "react-bootstrap/Tabs";
import Table from "react-bootstrap/Table";
import sanitizeStats from "./utils";
import "./stats-info.css";

function createTab(s) {
  let stat = JSON.parse(s);
  let columnId = stat["staattnum"];
  return (
    <Tab
      className="header"
      key={columnId}
      eventKey={columnId}
      title={"Column " + columnId}
    >
      <Table striped bordered hover variant="dark" responsive>
        <tbody>
          {Object.entries(stat).map(([k, v]) => (
            <tr>
              <th>{k}</th>
              <th>{JSON.stringify(v)}</th>
            </tr>
          ))}
        </tbody>
      </Table>
    </Tab>
  );
}

function ColumnTabs({ statsArray }) {
  console.log(statsArray, typeof statsArray);
  return (
    <div className="right-div">
      <Tabs defaultActiveKey="profile" className="mb-3">
        {statsArray.map((s) => createTab(s))}
      </Tabs>
    </div>
  );
}

export default ColumnTabs;
