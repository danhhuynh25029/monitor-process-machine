import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";;
import { CircularProgressbar } from "react-circular-progressbar";
import "react-circular-progressbar/dist/styles.css";
import { Col, Container, Row, Table } from "react-bootstrap";
import Chart from "react-apexcharts";

let ListProcess : any[] = [];
let ChartDonut : any;

function App() {
  const [ram, setRam] = useState(0);
  const [cpu, setCPU] = useState<any []>([]);
  const [process, setProcess] = useState([]);


  useEffect(() => {
    const interval = setInterval(async () => {
      setRam(await invoke("show_memory"));
      setCPU(await invoke("show_cpu"));
      setProcess(await invoke("show_process"));
    }, 1500)
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
     ListProcess = process.map((p) =>{
      return <tr>
        <td>{p[0]}</td>
        <td>{p[1]}</td>
        <td>{p[2]}</td>
        <td>{p[3]}</td>
      </tr>
  })

  },[process])


  useEffect(() => {
    if (cpu[0] != undefined && cpu[1] != undefined){
      const options = {
        chart: {
          id: "basic-bar"
        },
        xaxis: {
          categories: cpu[0]
        },
        colors : [ ({value} : {value : number}) => (value > 90) ? '#FF0000' : '#02DFDE'],
      }
      const series =  [
        {
          name: "CPU",
          data: cpu[1]
        }
      ]
      ChartDonut = <Chart width="400" type="bar" options={options} series={series} colors/>
    }
  },[cpu])
  

  return (
    <>
      <Container>
        <Row>
          <Col> <CircularProgressbar value={ram} text={`${ram}%`} /> Ram </Col>
          <Col>
             {ChartDonut}
          </Col>
        </Row>
        <Row>
          <Col>
            <Table>
              <thead>
                <tr>
                  <th>PID</th>
                  <th>Name</th>
                  <th>CPU Usage</th>
                  <th>Ram Usage</th>
                </tr>
              </thead>
              <tbody>
                {ListProcess}
              </tbody>
            </Table>
          </Col>
        </Row>
      </Container>
    </>
  );
}

export default App;
