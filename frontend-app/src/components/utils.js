import Table from "react-bootstrap/Table";
import Histogram from "./histogram";

function sanitizeStats(stats) {
  const cleanStats = new Map();
  cleanStats.set("Null Fraction", [stats["stanullfrac"], "stanullfrac"]);
  cleanStats.set("Average Entry Width (bytes)", [
    stats["stawidth"],
    "stawidth",
  ]);
  cleanStats.set("Distinct Elements", [stats["stadistinct"], "stadistinct"]);
  let sumMCVFreqs = 0;

  for (let i = 1; i <= 5; i++) {
    const stakindStr = "stakind" + i;
    const stavaluesStr = "stavalues" + i;
    const stanumbersStr = "stanumbers" + i;
    const stakind = stats[stakindStr];
    const stavalues = JSON.parse(stats[stavaluesStr]["data"]);
    const stanumbers = JSON.parse(stats[stanumbersStr]["data"]);
    if (stakind > 0) {
      switch (stakind) {
        case 1:
          // Most Common Values
          let valuesTable = (
            <div className="inset-table">
              <Table striped bordered hover size="sm" variant="dark">
                <thead>
                  <tr>
                    <th>Value</th>
                    <th>Frequency</th>
                  </tr>
                </thead>
                <tbody>
                  {stavalues.map((v, i) => (
                    <tr key={"value" + i}>
                      <th>{JSON.stringify(v)}</th>
                      <th>{stanumbers[i]}</th>
                    </tr>
                  ))}
                </tbody>
              </Table>
            </div>
          );
          sumMCVFreqs = stanumbers.reduce(
            (accumulator, currentValue) => accumulator + currentValue,
            0
          );
          cleanStats.set("Most Common Values", [valuesTable, stakindStr]);
          break;
        case 2:
          let histogram = (
            <Histogram
              width={500}
              height={200}
              data={stavalues}
              yValue={(1 - sumMCVFreqs) / (stavalues.length - 1)}
            />
          );
          cleanStats.set("Value Histogram (excluding Most Common Values)", [
            histogram,
            stakindStr,
          ]);
          break;
        case 3:
          cleanStats.set("Correlation", [stanumbers[0], stakindStr]);
          break;
        case 4:
          let elementsTable = (
            <Table striped bordered hover size="sm" variant="dark">
              <thead>
                <tr>
                  <th>Element</th>
                  <th>Frequency</th>
                </tr>
              </thead>
              <tbody>
                {stavalues.map((v, i) => (
                  <tr key={"value" + i}>
                    <th>{JSON.stringify(v)}</th>
                    <th>{stanumbers[i]}</th>
                  </tr>
                ))}
              </tbody>
            </Table>
          );
          cleanStats.set("Most Common Elements", [elementsTable, stakindStr]);
          break;
        default:
          break;
      }
    }
  }
  return cleanStats;
}

export default sanitizeStats;
