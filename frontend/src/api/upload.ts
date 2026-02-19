import {api} from "@/api/client.ts";
import {UploadRoutes} from "@/api/routes.ts";
import {formDataFromObject} from "@/api/apiHelpers.ts";

interface uploadResult {
    temp_id: string
}
export function uploadSingleFile(file: File): Promise<uploadResult> {
    const payload: any = {image:file}
    return api<uploadResult>(UploadRoutes.single(), {
        method: "POST",
        data: formDataFromObject(payload),
        headers: {"Content-Type": "multipart/form-data"},
    });
}
