import Tab from "react-bootstrap/Tab";
import Tabs from "react-bootstrap/Tabs";
import Table from "react-bootstrap/Table";
import sanitizeStats from "./utils";
import "./stats-info.css";

function createTab(s) {
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
          {[...sanitizeStats(stats).entries()].map(([k, v], idx) => (
            <tr key={k}>
              <th>{k}</th>
              <th>{v}</th>
            </tr>
          ))}
        </tbody>
      </Table>
    </Tab>
  );
}

function ColumnTabs({ statsArray }) {
  return (
    <div className="right-div">
      <Tabs defaultActiveKey="1" className="mb-3">
        {statsArray.map((s) => createTab(s))}
      </Tabs>
    </div>
  );
}

export default ColumnTabs;
