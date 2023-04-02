import { getApiURL } from '../utils/util';
import axios from 'axios';

export const axiosClient = axios.create({
    baseURL: `${getApiURL().href}api/v1/`,
    headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
    }
});

export function getDevices() {
    return axiosClient.get('device');
}

export function getDevice(id) {
    return axiosClient.get(`device/${id}`);
}

export function updateDevice(device) {
    return axiosClient.put(`device/${device.id}`, {
        data: device,
    });
}

export function createDevice(device) {
    return axiosClient.post(`device`, {
        data: device,
    });
}

export function deleteDevice(id) {
    return axiosClient.delete(`device/${id}`);
}
