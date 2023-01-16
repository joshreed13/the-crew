import { TricksPageData } from "../model";
import { TrickView } from "../Common";

export default function TricksPage({ data }: { data: TricksPageData }) {
    return (
        <div>
            {data.tricks.map((trickData) => (<TrickView data={trickData} />))}
        </div>
    );
}
