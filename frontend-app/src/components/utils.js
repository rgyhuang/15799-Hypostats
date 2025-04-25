import Table from "react-bootstrap/Table";

function sanitizeStats(stats) {
  const cleanStats = new Map();
  cleanStats.set("Null Fraction", stats["stanullfrac"]);
  cleanStats.set("Average Entry Width", stats["stawidth"]);
  cleanStats.set("Number of Distinct Elements", stats["stadistinct"]);

  for (let i = 1; i <= 5; i++) {
    let stakind = stats["stakind" + i];
    let stavalues = JSON.parse(stats["stavalues" + i]);
    let stanumbers = JSON.parse(stats["stanumbers" + i]);
    console.log(stavalues, stanumbers);
    if (stakind > 0) {
      switch (stakind) {
        case 1:
          // Most Common Values
          let valuesTable = (
            <Table striped bordered hover size="sm">
              {stavalues.map((v, i) => (
                <tr>
                  <th>{v}v</th>
                  <th>{stanumbers[i]}</th>
                </tr>
              ))}
            </Table>
          );
          cleanStats.set("Most Common Values", valuesTable);
          break;
        case 2:
          break;
        default:
          break;
      }
    }
  }
  return cleanStats;
}

export default sanitizeStats;
