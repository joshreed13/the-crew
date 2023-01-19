import { Solve, SolverPageData } from "../model";

export default function SolverPage({ data }: { data: SolverPageData }) {
    return (
        <div>
            <ol>
                {data.solves.map((solve: Solve) => (
                    <li key={solve.id}>
                        <span>#{solve.id} {result(solve)} <i>{solve.duration}ms</i></span>
                    </li>
                ))}
            </ol>
        </div>
    );
}

function result(solve: Solve): string {
    if (solve.success && solve.result) {
        return "Winnable";
    }
    else if (solve.success && !solve.result) {
        return "Not winnable";
    }
    else {
        return "Computation failed";
    }
}
